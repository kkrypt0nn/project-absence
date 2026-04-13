use std::collections::HashMap;
use std::vec;

use crate::database::node::Type;
use crate::event_bus::Event;
use crate::logger;
use crate::modules::Module;
use crate::session::Session;

pub struct ModuleInfrastructure {
    cloud_provider: HashMap<&'static str, Vec<&'static str>>,
    interesting_headers: Vec<&'static str>,
    security_headers: Vec<&'static str>,
}

impl ModuleInfrastructure {
    pub fn new() -> Self {
        ModuleInfrastructure {
            cloud_provider: HashMap::from([
                (
                    "amazon",
                    vec![
                        "header:x-amz-cf-id",
                        "header:x-amz-cf-pop",
                        "header:x-lae-region",
                        "via:cloudfront.net",
                    ],
                ),
                (
                    "azure",
                    vec![
                        "header:x-azure-ref",
                        "header:x-azure-originstatuscode",
                        "header:x-azure-internalerror",
                        "header:x-azure-externalerror",
                    ],
                ),
                ("heroku", vec!["server:heroku", "via:heroku-router"]),
                (
                    "vercel",
                    vec![
                        "header:x-vercel-cache",
                        "header:x-vercel-id",
                        "server:vercel",
                    ],
                ),
            ]),
            interesting_headers: vec![
                "server",
                "x-powered-by",
                "x-served-by",
                "cf-ray",
                "authorization",
                "set-cookie",
                "last-modified",
                "x-lae-region",
                "x-azure-debuginfo",
            ],
            security_headers: vec![
                "content-security-policy",
                "strict-transport-security",
                "x-content-type-options",
                "x-frame-options",
                "referrer-policy",
                "permissions-policy",
                "tls-version",
                "tls-cipher-name",
            ],
        }
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
        let headers = &fetched_data.response.headers;

        if let Some(tls_data) = &tls {
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
                logger::println(self.name(), format!("Gathered TLS data for {}", domain));
            }
        }

        let mut interesting_headers = HashMap::new();
        let mut security_headers = HashMap::new();
        for (name, value) in headers {
            if self.interesting_headers.contains(&name.as_str()) {
                interesting_headers.insert(name, value);
            }
            if self.security_headers.contains(&name.as_str()) {
                security_headers.insert(name, value);
            }
        }

        let cloud_provider = self.cloud_provider.iter().find_map(|(provider, checks)| {
            checks.iter().find_map(|check| {
                let (prefix, value) = check.split_once(':')?;
                match prefix {
                    "header" => headers
                        .iter()
                        .any(|(name, _)| name == value)
                        .then_some(*provider),
                    _ => headers.iter().find(|(name, _)| name == prefix).and_then(
                        |(_, header_value)| header_value.contains(value).then_some(*provider),
                    ),
                }
            })
        });

        if let Some(parent) = session
            .get_database()
            .search(Type::Domain, domain.to_string())
        {
            parent.add_data(
                "cloud_provider".to_string(),
                serde_json::to_value(cloud_provider).unwrap(),
            );
            parent.add_data(
                "interesting_headers".to_string(),
                serde_json::to_value(interesting_headers).unwrap(),
            );
            parent.add_data(
                "security_headers".to_string(),
                serde_json::to_value(security_headers).unwrap(),
            );
            logger::println(
                self.name(),
                format!(
                    "Gathered interesting and security headers for {}{}",
                    domain,
                    if let Some(cloud_provider) = cloud_provider {
                        format!(
                            ", as well as the potential cloud provider ({})",
                            cloud_provider
                        )
                    } else {
                        "".to_string()
                    }
                ),
            );
        }

        Ok(())
    }
}
