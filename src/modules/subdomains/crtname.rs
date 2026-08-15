use std::sync::Arc;

use reqwest::StatusCode;
use serde_json::Value;

use reqwest::header::USER_AGENT;

use crate::database::node::{Node, Type};
use crate::event_bus::Event;
use crate::modules::Module;
use crate::session::Session;
use crate::{config, helpers, logger};

pub struct Runner {
    _config: config::SubdomainsCrtNameConfig,
}

impl Runner {
    pub fn new(config: config::SubdomainsCrtNameConfig) -> Self {
        Runner { _config: config }
    }
}

impl Module for Runner {
    fn name(&self) -> String {
        String::from("discovery:subdomains:crtname")
    }

    fn description(&self) -> String {
        String::from(
            "This module will perform a passive discovery of new domains by using crt.name",
        )
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("discovered:domain")]
    }

    fn execute(&self, session: Arc<Session>, event: &Event) -> Result<(), String> {
        let domain = match event {
            Event::DiscoveredDomain(domain) => domain,
            _ => return Err("Received wrong event, exiting module".to_string()),
        };

        // Only apex allowed
        if domain.split('.').count() != 2 {
            return Ok(());
        }

        let response = session
            .get_http_client()
            .get(format!("https://crt.name/v1/search?apex={}", domain))
            .header(USER_AGENT, helpers::ua::get_random())
            .send();
        match response {
            Ok(response) => {
                let status = response.status();
                if status != StatusCode::OK {
                    return Err(format!("crt.name returned status code {}", status));
                }

                let body = response
                    .text()
                    .map_err(|_| "Failed to read the body of crt.name")?;

                for line in body.lines() {
                    let subdomain = line.trim();
                    if subdomain.is_empty() || subdomain == domain {
                        continue;
                    }
                    if session
                        .get_state()
                        .has_discovered_domain(subdomain.to_string())
                    {
                        continue;
                    }

                    logger::println(
                        self.name(),
                        format!("Discovered '{}' as a new subdomain", subdomain),
                    );

                    if let Some(parent) =
                        session.get_database().search(Type::Domain, domain.clone())
                    {
                        let mut new_node = Node::new(Type::Domain, subdomain.to_string());
                        if let Some(ip_addr) = helpers::network::get_ip_addr(subdomain) {
                            new_node
                                .add_data(String::from("ip"), Value::String(ip_addr.to_string()));
                            if let Some(geoinfo) = helpers::network::geolocate_ip(ip_addr) {
                                new_node.add_data(String::from("geoinfo"), geoinfo.into())
                            }
                        }
                        parent.connect(new_node);
                    }
                    session.get_state().discover_domain(subdomain.to_string());
                    session.publish(Event::DiscoveredDomain(subdomain.to_string()));
                }

                Ok(())
            }
            Err(_) => Err("Failed to perform a request to crt.name".to_string()),
        }
    }
}
