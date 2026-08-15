use std::collections::HashMap;
use std::sync::Arc;

use crate::database::node::Type;
use crate::event_bus::Event;
use crate::modules::Module;
use crate::session::Session;
use crate::{flags, logger};

pub struct ModuleDomainTakeover {
    platforms: HashMap<&'static str, &'static str>,
}

impl ModuleDomainTakeover {
    pub fn new() -> Self {
        ModuleDomainTakeover {
            platforms: HashMap::from([
                (
                    "github",
                    "<p><strong>There isn't a GitHub Pages site here.</strong></p>",
                ),
                ("glitch", "<h1>Well, you found a glitch.</h1>"),
                (
                    "heroku",
                    "<iframe src=\"//www.herokucdn.com/error-pages/no-such-app.html\"></iframe>",
                ),
                ("netlify", "Not Found - Request ID: "),
                ("railway", "Application not found"),
                ("replit", "Not Found"),
                ("vercel", "The deployment could not be found on Vercel."),
            ]),
        }
    }

    fn name_with_platform(&self, platform: &str) -> String {
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

    fn execute(&self, session: Arc<Session>, event: &Event) -> Result<(), String> {
        let fetched_data = match event {
            Event::DomainFetched(fetched_data) => fetched_data,
            _ => return Err("Received wrong event, exiting module".to_string()),
        };
        let domain = &fetched_data.domain;
        let body = &fetched_data.response.body;

        for (&platform, content) in self.platforms.iter() {
            if body.contains(content) {
                if let Some(parent) = session.get_database().search(Type::Domain, domain.clone()) {
                    parent.add_data(String::from("possible_takeover"), platform.into());
                    parent.add_flag(flags::domain::POSSIBLE_TAKEOVER);
                }
                logger::println(
                    self.name_with_platform(platform),
                    format!("Domain takeover possible for '{}'", domain),
                );
                break;
            }
        }

        Ok(())
    }
}
