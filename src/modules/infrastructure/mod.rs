use crate::database::node::Type;
use crate::event_bus::Event;
use crate::logger;
use crate::modules::Module;
use crate::session::Session;

pub struct ModuleInfrastructure;

impl ModuleInfrastructure {
    pub fn new() -> Self {
        ModuleInfrastructure
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
        vec![String::from("domain:fetched")]
    }

    fn execute(&self, session: &Session, event: &Event) -> Result<(), String> {
        let fetched_data = match event {
            Event::DomainFetched(fetched_data) => fetched_data,
            _ => return Err("Received wrong event, exiting module".to_string()),
        };
        let domain = &fetched_data.domain;
        let tls = &fetched_data.response.tls;

        if let Some(tls_data) = &tls {
            logger::println(self.name(), format!("Gathered TLS data for {}", domain));

            for san in &tls_data.san {
                if !session.get_state().has_discovered_domain(san.to_string()) {
                    session.publish(Event::DiscoveredDomain(san.clone()));
                }
            }

            if let Some(parent) = session
                .get_database()
                .search(Type::Domain, domain.to_string())
            {
                parent.add_data("tls".to_string(), serde_json::to_value(tls_data).unwrap());
            }
        }

        Ok(())
    }
}
