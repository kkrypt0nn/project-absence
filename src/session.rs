use std::fs::{File, create_dir_all};
use std::io::{Error, Write};
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::{env, thread};

use reqwest::blocking::{Client, ClientBuilder};

use crate::event_bus::{self, EventBus};
use crate::modules::Module;
use crate::{args, config, database, debug, logger, modules, state};

macro_rules! add_runner {
    ($enabled_runners:expr, $runners_vec:expr, $name:expr, $cfg:expr, $constructor:path) => {
        if $enabled_runners.iter().any(|r| r.to_string() == $name) {
            $runners_vec.push(Box::new($constructor($cfg.clone().unwrap_or_default())));
        }
    };
}

pub struct Session {
    args: args::Args,
    bus: EventBus,
    config: config::Config,
    database: Arc<Mutex<database::Database>>,
    state: Arc<state::State>,
    http_client: Client,
    shutdown: Arc<(Mutex<bool>, Condvar)>,
}

impl Session {
    pub fn new(args: args::Args, config: config::Config) -> Arc<Self> {
        let domain_clone = args.clone().domain;
        let is_verbose = args.verbose;
        let is_debug = args.debug;
        Arc::new(Session {
            args,
            bus: EventBus::new(),
            config,
            database: Arc::new(Mutex::new(database::Database::new(
                database::node::Node::new(database::node::Type::Domain, domain_clone),
            ))),
            state: Arc::new(state::State::new(is_verbose, is_debug)),
            http_client: ClientBuilder::new()
                .tls_info(true)
                .build()
                .expect("Client::new()"),
            shutdown: Arc::new((Mutex::new(false), Condvar::new())),
        })
    }

    pub fn get_args(&self) -> &args::Args {
        &self.args
    }

    pub fn get_database(&self) -> MutexGuard<'_, database::Database> {
        self.database.lock().unwrap()
    }

    pub fn get_database_arc(&self) -> Arc<Mutex<database::Database>> {
        Arc::clone(&self.database)
    }

    pub fn get_state(&self) -> Arc<state::State> {
        Arc::clone(&self.state)
    }

    pub fn get_http_client(&self) -> &Client {
        &self.http_client
    }

    pub fn publish(&self, event: event_bus::Event) {
        self.bus.publish(&event);
    }

    fn output_results(&self) -> Result<(), Error> {
        #[cfg(feature = "clipboard")]
        if self.get_args().clipboard {
            use arboard::Clipboard;

            let mut clipboard = Clipboard::new().unwrap();
            if clipboard
                .set_text(self.get_database().get_as_pretty_json())
                .is_ok()
            {
                logger::info(
                    "",
                    "Successfully copied the resulting JSON database to the clipboard",
                )
            }
        }

        if self.get_state().is_debug() {
            debug::database::render_compact(&mut self.get_database());
        }

        let home_dir = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .unwrap_or_else(|_| String::from(""));
        let result_path = &self.get_args().output;
        let expanded_result_path = if result_path.starts_with("~") {
            let mut expanded_path = result_path.clone();
            expanded_path.replace_range(0..1, &home_dir);
            expanded_path
        } else {
            result_path.clone()
        };

        // JSON Result
        let json_result_path = PathBuf::from(format!("{}/results.json", expanded_result_path));
        if create_dir_all(json_result_path.parent().unwrap()).is_ok() {
            let mut file_result = File::create(json_result_path.clone())?;
            if file_result
                .write_all(self.get_database().get_as_pretty_json().as_bytes())
                .is_ok()
            {
                logger::info(
                    "",
                    format!(
                        "Successfully wrote the JSON result in '{}'",
                        json_result_path.display()
                    ),
                )
            };
        }

        // Markdown Result
        let markdown_result_path = PathBuf::from(format!("{}/results.md", expanded_result_path));
        if create_dir_all(markdown_result_path.parent().unwrap()).is_ok() {
            let mut file_result = File::create(markdown_result_path.clone())?;
            let domains_data = self.get_database().get_root().to_markdown();
            let content = format!(
                "# Analysis Report for '{}'\n\n## Domains\n\n{}",
                &self.get_args().domain,
                domains_data
            );
            if file_result.write_all(content.as_bytes()).is_ok() {
                logger::info(
                    "",
                    format!(
                        "Successfully wrote the Markdown report in '{}'",
                        markdown_result_path.display()
                    ),
                )
            };
        }

        Ok(())
    }

    pub fn register_module<T: Module + Send + Sync + 'static>(self: &Arc<Self>, module: T) {
        let module_arc = Arc::new(Box::new(module));
        for event in module_arc.subscribers() {
            let session_clone = Arc::clone(self);
            let module_clone = Arc::clone(&module_arc);
            self.bus.subscribe(event.to_string().as_str(), move |e| {
                let session = Arc::clone(&session_clone);
                let module_clone = Arc::clone(&module_clone);

                session.get_state().increment_tasks();
                if session.get_state().is_debug_or_verbose() {
                    logger::debug(
                        "task",
                        format!(
                            "Task added, tasks now are at {}",
                            session.get_state().active_tasks_count()
                        ),
                    );
                }

                let permit = session.get_state().get_semaphore_permit();
                let event_clone = e.clone();

                if session.get_state().is_debug_or_verbose() {
                    logger::trace(
                        "bus:run",
                        format!(
                            "Running module {} as the event {:?} has been emitted",
                            module_clone.name(),
                            e,
                        ),
                    );
                }

                thread::spawn(move || {
                    if let Err(e) = module_clone.execute(&session, &event_clone) {
                        logger::error(module_clone.name(), e);
                    }

                    session.publish(event_bus::Event::FinishedTask);
                    drop(permit);
                });
            });
        }
    }

    // TODO: This deserves some cleanup
    // TODO: Include in the cleanup a way to prevent always having to do `config.clone()`, while also retaining a clean use of the config in the modules
    // TODO: Include in the cleanup a way to not have to add the runners manually? Maybe some register_runner macro for the module?
    pub fn register_config_modules(self: &Arc<Self>) {
        self.register_module(modules::ready::ModuleReady::new());
        self.register_module(modules::request::ModuleRequest::new());

        // Load Lua module
        // TODO: Allow multiple Lua modules in the future. For the current PoC, one is fine.
        if let Some(script) = &self.args.script {
            let lua_module =
                modules::scripting::Scripting::new(script).expect("Failed to load Lua module");
            self.register_module(lua_module);
        }

        if let Some(endpoints_cfg) = &self.config.endpoints {
            let enabled_runners = endpoints_cfg.enabled_runners.as_deref().unwrap_or(&[]);
            let mut runners: Vec<Box<dyn Module>> = Vec::new();

            add_runner!(
                enabled_runners,
                runners,
                "wayback_machine",
                &endpoints_cfg.wayback_machine,
                modules::endpoints::wayback_machine::Runner::new
            );

            if !runners.is_empty() {
                self.register_module(modules::endpoints::EndpointDiscoveryModule::new(runners));
            }
        }

        if let Some(domain_takeover_cfg) = &self.config.domain_takeover
            && domain_takeover_cfg.enabled
        {
            self.register_module(modules::domain_takeover::ModuleDomainTakeover::new());
        }

        if let Some(emails_cfg) = &self.config.emails {
            let enabled_runners = emails_cfg.enabled_runners.as_deref().unwrap_or(&[]);
            let mut runners: Vec<Box<dyn Module>> = Vec::new();

            add_runner!(
                enabled_runners,
                runners,
                "dork",
                &emails_cfg.dork,
                modules::emails::dork::Runner::new
            );

            if !runners.is_empty() {
                self.register_module(modules::emails::EmailDiscoveryModule::new(runners));
            }
        }

        if let Some(subdomains_cfg) = &self.config.subdomains {
            let enabled_runners = subdomains_cfg.enabled_runners.as_deref().unwrap_or(&[]);
            let mut runners: Vec<Box<dyn Module>> = Vec::new();

            add_runner!(
                enabled_runners,
                runners,
                "dork",
                &subdomains_cfg.dork,
                modules::subdomains::dork::Runner::new
            );
            add_runner!(
                enabled_runners,
                runners,
                "crtsh",
                &subdomains_cfg.crtsh,
                modules::subdomains::crtsh::Runner::new
            );

            if !runners.is_empty() {
                self.register_module(modules::subdomains::SubdomainDiscoveryModule::new(runners));
            }
        }

        if let Some(infrastructure_cfg) = &self.config.infrastructure
            && infrastructure_cfg.enabled
        {
            self.register_module(modules::infrastructure::ModuleInfrastructure::new());
        }

        if let Some(dns_cfg) = &self.config.dns
            && dns_cfg.enabled
        {
            self.register_module(modules::dns::ModuleDns::new(dns_cfg.clone()));
        }

        if let Some(technologies_cfg) = &self.config.technologies
            && technologies_cfg.enabled
        {
            self.register_module(modules::technologies::ModuleTechnologies::new());
        }

        if let Some(files_cfg) = &self.config.files {
            let enabled_runners = files_cfg.enabled_runners.as_deref().unwrap_or(&[]);
            let mut runners: Vec<Box<dyn Module>> = Vec::new();

            add_runner!(
                enabled_runners,
                runners,
                "dork",
                &files_cfg.dork,
                modules::files::dork::Runner::new
            );

            if !runners.is_empty() {
                self.register_module(modules::files::FileDiscoveryModule::new(runners));
            }
        }
    }

    pub fn run(self: &Arc<Self>) -> Result<(), Error> {
        let session = Arc::clone(self);
        let state = session.get_state();
        self.bus.subscribe("finished:task", move |_| {
            state.decrement_tasks();

            if state.is_debug_or_verbose() {
                logger::debug(
                    "task",
                    format!(
                        "Task finished, tasks now are at {}",
                        state.active_tasks_count()
                    ),
                );
            }

            if state.active_tasks_count() == 0 {
                if let Err(e) = session.output_results() {
                    logger::error("session", e.to_string());
                }

                let (mutex, condvar) = &*session.shutdown;
                *mutex.lock().unwrap() = true;
                condvar.notify_all();
            }
        });

        if self.get_state().is_debug_or_verbose() {
            thread::spawn({
                let state_clone = Arc::clone(&self.get_state());
                move || {
                    state_clone.actively_report();
                }
            });
        }

        self.publish(event_bus::Event::Ready);
        self.publish(event_bus::Event::DiscoveredDomain(
            self.get_args().domain.clone(),
        ));

        let (mutex, condvar) = &*self.shutdown;
        let _shutdown_guard = condvar
            .wait_while(mutex.lock().unwrap(), |shutdown| !*shutdown)
            .unwrap();

        Ok(())
    }
}
