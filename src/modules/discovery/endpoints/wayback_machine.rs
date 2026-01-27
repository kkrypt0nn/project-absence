use std::collections::HashSet;
use std::time::Duration;
use std::vec;

use reqwest::StatusCode;
use reqwest::header::USER_AGENT;

use crate::database::node::{Node, Type};
use crate::event_bus::Event;
use crate::modules::Module;
use crate::session::Session;
use crate::{config, helpers, logger};

pub struct Runner {
    config: config::EndpointsWaybackMachineConfig,
}

impl Runner {
    pub fn new(config: config::EndpointsWaybackMachineConfig) -> Self {
        Runner { config }
    }
}

impl Module for Runner {
    fn name(&self) -> String {
        String::from("discovery:endpoint:wayback_machine")
    }

    fn description(&self) -> String {
        String::from("Uses the Wayback Machine to find (previously) existing endpoints")
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("discovered:domain")]
    }

    fn execute(&self, session: &Session, event: &Event) -> Result<(), String> {
        let domain = match event {
            Event::DiscoveredDomain(domain) => domain,
            _ => return Err("Received wrong event, exiting module".to_string()),
        };

        let timeout_seconds = self.config.timeout.unwrap_or(30);

        let response = session
            .get_http_client()
            .get(format!("https://web.archive.org/cdx/search/cdx?url={}/*&output=txt&collapse=urlkey&fl=original&page=/", domain))
            .header(USER_AGENT, helpers::ua::get_random())
            .timeout(Duration::from_secs(timeout_seconds))
            .send();
        match response {
            Ok(response) => {
                let status = response.status();
                if status != StatusCode::OK {
                    return Err(format!("Wayback Machine returned status code {}", status));
                }

                let text = response.text().unwrap_or_default();
                let mut seen = HashSet::new();
                let endpoints: Vec<&str> = text.lines().filter(|line| seen.insert(*line)).filter(|item| !session.get_state().has_discovered_endpoint(item.to_string())).collect();
                logger::println(self.name(), format!("Discovered $[effect:bold]{}$[effect:reset] new endpoints for the '{}' domain on the Wayback Machine", endpoints.len(), &domain));

                for endpoint in endpoints {
                    if let Some(parent) =
                        session.get_database().search(Type::Domain, domain.clone())
                    {
                        parent.connect(Node::new(Type::Endpoint, endpoint.to_string()));
                    }
                    session.get_state().discover_endpoint(endpoint.to_string());
                }

                Ok(())
            },
            Err(_) => {
                Err("Failed performing a request to the Wayback Machine; is it down or a big website where the timeout may be too small?".to_string())
            }
        }
    }
}
