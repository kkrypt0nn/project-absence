use std::collections::HashMap;

use crate::database::node::Type;
use crate::event_bus::Event;
use crate::modules::Module;
use crate::session::Session;
use crate::{flags, logger};

pub struct ModuleDomainTakeover {
    platforms: HashMap<String, String>,
}

impl ModuleDomainTakeover {
    pub fn new() -> Self {
        ModuleDomainTakeover {
            platforms: HashMap::from([
                (
                    String::from("github"),
                    String::from("<p><strong>There isn't a GitHub Pages site here.</strong></p>"),
                ),
                (
                    String::from("glitch"),
                    String::from("<h1>Well, you found a glitch.</h1>"),
                ),
                (
                    String::from("heroku"),
                    String::from(
                        "<iframe src=\"//www.herokucdn.com/error-pages/no-such-app.html\"></iframe>",
                    ),
                ),
                (
                    String::from("netlify"),
                    String::from("Not Found - Request ID: "),
                ),
                (
                    String::from("railway"),
                    String::from("Application not found"),
                ),
                (String::from("replit"), String::from("Not Found")),
                (
                    String::from("vercel"),
                    String::from("The deployment could not be found on Vercel."),
                ),
            ]),
        }
    }

    fn name_with_platform(&self, platform: String) -> String {
        format!("{}({})", self.name(), platform)
    }
}

impl Module for ModuleDomainTakeover {
    fn name(&self) -> String {
        String::from("domain_takeover")
    }

    fn description(&self) -> String {
        String::from(
            "This module checks for the content of a domain to know whether it can be taken over",
        )
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("domain:fetched")]
    }

    fn execute(&self, session: &Session, event: &Event) -> Result<(), String> {
        let fetched_data = match event {
            Event::DomainFetched(fetched_data) => fetched_data,
            _ => return Err("Received wrong event, exiting module".to_string()),
        };
        let domain = &fetched_data.domain;
        let body = &fetched_data.response.body;

        for (platform, content) in self.platforms.iter() {
            if body.contains(content) {
                if let Some(parent) = session.get_database().search(Type::Domain, domain.clone()) {
                    parent.add_data(
                        String::from("possible_takeover"),
                        platform.to_string().into(),
                    );
                    parent.add_flag(flags::domain::POSSIBLE_TAKEOVER);
                }
                logger::println(
                    self.name_with_platform(platform.to_string()),
                    format!("Domain takeover possible for '{}'", domain),
                );
                break;
            }
        }

        Ok(())
    }
}
