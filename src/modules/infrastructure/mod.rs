use std::sync::Mutex;

use reqwest::header::USER_AGENT;

use crate::database::node::Type;
use crate::event_bus::Event;
use crate::modules::Module;
use crate::session::Session;
use crate::{helpers, logger};

mod tls;

pub struct ModuleInfrastructure {
    processed_domains: Mutex<Vec<String>>,
}

impl ModuleInfrastructure {
    pub fn new() -> Self {
        ModuleInfrastructure {
            processed_domains: Mutex::new(Vec::new()),
        }
    }

    pub fn process(&self, domain: &str) {
        self.processed_domains
            .lock()
            .unwrap()
            .push(domain.to_string())
    }

    pub fn has_processed(&self, domain: &str) -> bool {
        self.processed_domains
            .lock()
            .unwrap()
            .contains(&domain.to_string())
    }
}

impl Module for ModuleInfrastructure {
    fn name(&self) -> String {
        String::from("infrastructure")
    }

    fn description(&self) -> String {
        String::from(
            "This module gets some information about the infrastructure of the domain, such as possible web-server, CDN used, etc.",
        )
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("discovered:domain")]
    }

    fn execute(&self, session: &Session, event: &Event) -> Result<(), String> {
        let domain = match event {
            Event::DiscoveredDomain(domain) => domain,
            _ => {
                return Err("Received wrong event, exiting module".to_string());
            }
        };

        if self.has_processed(domain) {
            return Ok(());
        }
        self.process(domain);

        let response = match session
            .get_http_client()
            .get(format!("https://{}", domain))
            .header(USER_AGENT, helpers::ua::get_random())
            .send()
        {
            Ok(r) => r,
            Err(e) => {
                logger::error(self.name(), format!("Failed to request {}: {}", domain, e));
                return Ok(());
            }
        };

        if let Some(tls_data) = tls::extract(&response) {
            logger::println(self.name(), format!("Gathered TLS data for {}", domain));

            if let Some(san_array) = tls_data.get("san").and_then(|v| v.as_array()) {
                for san in san_array.iter().filter_map(|v| v.as_str()) {
                    session.publish(Event::DiscoveredDomain(san.to_string()));
                }
            }

            if let Some(parent) = session
                .get_database()
                .search(Type::Domain, domain.to_string())
            {
                parent.add_data("tls".to_string(), tls_data);
            }
        }

        Ok(())
    }
}
