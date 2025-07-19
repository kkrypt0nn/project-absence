use std::fs::{File, create_dir_all};
use std::io::{Error, Write};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::{env, thread};

#[cfg(feature = "clipboard")]
use clipboard::{ClipboardContext, ClipboardProvider};

use reqwest::blocking::Client;

use crate::modules::Module;
use crate::{args, config, database, debug, events, logger, modules, state};

macro_rules! add_runner {
    ($enabled_runners:expr, $runners_vec:expr, $name:expr, $cfg_opt:expr, $constructor:path) => {
        if $enabled_runners.iter().any(|r| r.to_string() == $name) {
            if let Some(cfg) = $cfg_opt {
                $runners_vec.push(Box::new($constructor(cfg.clone())));
            }
        }
    };
}

pub struct Session {
    args: args::Args,
    config: config::Config,
    database: Arc<Mutex<database::Database>>,
    state: Arc<state::State>,
    http_client: Client,

    sender: SyncSender<events::Type>,
    receiver: Arc<Mutex<Receiver<events::Type>>>,

    modules: Mutex<Vec<Arc<Box<dyn Module>>>>,
}

impl Session {
    pub fn new(
        args: args::Args,
        config: config::Config,
        sender: SyncSender<events::Type>,
        receiver: Receiver<events::Type>,
    ) -> Arc<Self> {
        let domain_clone = args.clone().domain;
        let is_verbose = args.verbose;
        let is_debug = args.debug;
        Arc::new(Session {
            args,
            config,
            database: Arc::new(Mutex::new(database::Database::new(
                database::node::Node::new(database::node::Type::Domain, domain_clone),
            ))),
            state: Arc::new(state::State::new(is_verbose, is_debug)),
            http_client: Client::new(),

            sender,
            receiver: Arc::new(Mutex::new(receiver)),

            modules: Mutex::new(Vec::new()),
        })
    }

    pub fn get_args(&self) -> &args::Args {
        &self.args
    }

    pub fn get_database(&self) -> MutexGuard<database::Database> {
        self.database.lock().unwrap()
    }

    pub fn get_state(&self) -> Arc<state::State> {
        Arc::clone(&self.state)
    }

    pub fn get_http_client(&self) -> &Client {
        &self.http_client
    }

    pub fn register_module<T: Module + Send + Sync + 'static>(&self, module: T) {
        if self.get_state().is_debug_or_verbose() {
            logger::info("", format!("Registered module {}", module.name()))
        }
        let mut modules = self.modules.lock().unwrap();
        modules.push(Arc::new(Box::new(module)));
    }

    // TODO: This deserves some cleanup
    // TODO: Include in the cleanup a way to prevent always having to do `config.clone()`, while also retaining a clean use of the config in the modules
    // TODO: Include in the cleanup a way to not have to add the runners manually? Maybe some register_runner macro for the module?
    pub fn register_config_modules(&self) {
        self.register_module(modules::ready::ModuleReady::new());

        // Load Lua module
        // TODO: Allow multiple Lua modules in the future. For the current PoC, one is fine.
        if let Some(script) = &self.args.script {
            let lua_module =
                modules::scripting::Scripting::new(script).expect("Failed to load Lua module");
            self.register_module(lua_module);
        }

        if let Some(domain_takeover_cfg) = &self.config.domain_takeover {
            if domain_takeover_cfg.enabled {
                self.register_module(modules::domain_takeover::ModuleDomainTakeover::new());
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
                modules::discovery::subdomains::dork::Runner::new
            );
            add_runner!(
                enabled_runners,
                runners,
                "crtsh",
                &subdomains_cfg.crtsh,
                modules::discovery::subdomains::crtsh::Runner::new
            );

            if !runners.is_empty() {
                self.register_module(
                    modules::discovery::subdomains::SubdomainDiscoveryModule::new(runners),
                );
            }
        }
    }

    pub fn emit(&self, event: events::Type) {
        let event_name = event.to_string();
        if let Err(e) = self.sender.send(event) {
            logger::error(
                "emit",
                format!("Failed emitting the '{}' event: {}", event_name, e),
            );
        };
    }

    pub fn run(self: Arc<Self>) -> Result<(), Error> {
        if self.get_state().is_debug_or_verbose() {
            thread::spawn({
                let state_clone = Arc::clone(&self.get_state());
                move || {
                    state_clone.actively_report();
                }
            });
        }
        self.emit(events::Type::Ready);
        self.emit(events::Type::DiscoveredDomain(
            self.get_args().domain.clone(),
        ));

        while let Ok(event) = self.receiver.lock().unwrap().recv() {
            if event == events::Type::FinishedTask {
                self.get_state().decrement_tasks();
                if self.get_state().is_debug() {
                    logger::debug(
                        event.to_string(),
                        format!(
                            "A task has finished execution, current running tasks: {}",
                            self.get_state().active_tasks_count()
                        ),
                    );
                }
                if self.get_state().active_tasks_count() == 0 {
                    #[cfg(feature = "clipboard")]
                    if self.get_args().clipboard {
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        if ctx
                            .set_contents(self.get_database().get_as_pretty_json())
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
                    let json_result_path =
                        PathBuf::from(format!("{}/results.json", expanded_result_path));
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
                    let markdown_result_path =
                        PathBuf::from(format!("{}/results.md", expanded_result_path));
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

                    break;
                }
            }

            let modules = self.modules.lock().unwrap();
            for module in &*modules {
                if module.subscribers().iter().any(|sub_event| {
                    matches!(
                        (sub_event, &event),
                        (events::Type::Ready, events::Type::Ready)
                            | (
                                events::Type::DiscoveredDomain(_),
                                events::Type::DiscoveredDomain(_)
                            )
                            | (events::Type::OpenPort(_, _), events::Type::OpenPort(_, _))
                    )
                }) {
                    thread::spawn({
                        self.get_state().increment_tasks();
                        let permit = self.get_state().get_semaphore_permit();
                        let context = modules::get_context_for_event(&event);
                        let module_clone = Arc::clone(module);
                        let session_clone = Arc::clone(&self);
                        move || {
                            if let Err(e) = module_clone.execute(&session_clone, context) {
                                logger::error(module_clone.name(), e)
                            }
                            session_clone.emit(events::Type::FinishedTask);
                            drop(permit);
                        }
                    });
                }
            }
        }

        Ok(())
    }
}
