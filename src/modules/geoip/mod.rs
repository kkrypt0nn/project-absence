use std::sync::Arc;

use crate::{
    database::node::{NodeMetadata, Type},
    event_bus::Event,
    helpers, logger,
    modules::Module,
    session::Session,
};

pub struct ModuleGeoIp;

impl Default for ModuleGeoIp {
    fn default() -> Self {
        Self
    }
}

impl Module for ModuleGeoIp {
    fn name(&self) -> &'static str {
        "geoip"
    }

    fn description(&self) -> String {
        String::from(
            "This module gets the location of the IP behind the domain, based on ip-api.com",
        )
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("domain:fetched")]
    }

    fn execute(&self, session: Arc<Session>, event: &Event) -> Result<(), String> {
        let Event::DomainFetched(fetched_data) = event else {
            return Err("Received wrong event, exiting module".to_string());
        };
        let domain = &fetched_data.domain;

        if let Some(node) = session.get_database().search(Type::Domain, domain.clone()) {
            if let Some(ip_addr) = helpers::network::get_ip_addr(&domain) {
                if let NodeMetadata::Domain(metadata) = &mut node.metadata {
                    metadata.ip = Some(ip_addr.to_string());
                    if let Some(geoinfo) = helpers::network::geolocate_ip(ip_addr) {
                        metadata.geoinfo = Some(geoinfo);
                    }
                }

                logger::println(self.name(), format!("Stored geoip data for {domain}"));
            }
        }

        Ok(())
    }
}
