use std::sync::Arc;

use crate::{
    database::node::{Node, NodeMetadata, Type},
    event_bus::Event,
    flags, helpers, logger,
    modules::Module,
    session::Session,
};

pub struct ModuleTls;

impl ModuleTls {
    fn check_tls_version(version: &str) -> Option<(&'static str, usize)> {
        match version {
            "TLS 1.0" => Some(("TLS 1.0 is deprecated", flags::tls::DEPRECATED_TLS)),
            "TLS 1.1" => Some(("TLS 1.1 is deprecated", flags::tls::DEPRECATED_TLS)),
            _ => None,
        }
    }

    fn check_validity(expires_in_seconds: i64) -> Option<(&'static str, usize)> {
        if expires_in_seconds < 0 {
            Some(("Certificate has expired", flags::tls::EXPIRED))
        } else if expires_in_seconds < 60 * 60 * 24 * 7 {
            Some((
                "Certificate expires within 7 days",
                flags::tls::EXPIRING_SOON,
            ))
        } else {
            None
        }
    }
}

impl Default for ModuleTls {
    fn default() -> Self {
        Self
    }
}

impl Module for ModuleTls {
    fn name(&self) -> &'static str {
        "tls"
    }

    fn description(&self) -> String {
        String::from(
            "This module extracts TLS certificate information and identifies weak algorithms, deprecated protocols and certificate validity issues",
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
        let Some(tls) = &fetched_data.response.tls else {
            return Ok(());
        };

        let mut findings: Vec<(&str, usize)> = Vec::new();

        if let Some(finding) = Self::check_tls_version(&tls.version) {
            findings.push(finding);
        }
        if let Some(finding) = Self::check_validity(tls.expires_in_seconds) {
            findings.push(finding);
        }
        if tls.subject == tls.issuer {
            findings.push((
                "Certificate appears to be self-signed",
                flags::tls::SELF_SIGNED,
            ));
        }

        let mut combined_flags = 0;

        for (finding, flag) in &findings {
            combined_flags |= flag;
            logger::println(self.name(), finding.to_string());
        }

        let mut database = session.get_database();
        let mut parent = database.search(Type::Domain, domain.to_string());

        if let Some(parent) = parent.as_mut() {
            parent.add_flag(combined_flags);

            if let NodeMetadata::Domain(metadata) = &mut parent.metadata {
                metadata.tls = Some(tls.clone());
                logger::println(self.name(), format!("Stored TLS data for {domain}"));
            }
        }

        for san in &tls.san {
            let san = san.trim().to_lowercase();
            if san.is_empty() || san == *domain || san.starts_with("*.") {
                continue;
            }
            if !session.get_state().try_discover_domain(san.clone()) {
                continue;
            }

            logger::println(
                self.name(),
                format!("Discovered '{san}' from TLS certificate SAN"),
            );

            if let Some(parent) = parent.as_mut() {
                let mut new_node = Node::new(Type::Domain, san.clone());
                if let Some(ip_addr) = helpers::network::get_ip_addr(&san) {
                    if let NodeMetadata::Domain(metadata) = &mut new_node.metadata {
                        metadata.ip = Some(ip_addr.to_string());
                        if let Some(geoinfo) = helpers::network::geolocate_ip(ip_addr) {
                            metadata.geoinfo = Some(geoinfo);
                        }
                    }
                }
                parent.connect(new_node);
            }

            session.publish(Event::DiscoveredDomain(san));
        }

        Ok(())
    }
}
