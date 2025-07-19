use std::collections::HashMap;
use std::fmt;

use regex::Regex;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::database::node::{Node, Type};
use crate::modules::{Context, Module};
use crate::session::Session;
use crate::{config, events, helpers, logger};

#[derive(
    Copy, Clone, Debug, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash,
)]
#[serde(rename_all = "lowercase")]
pub enum SearchEngine {
    Ecosia,
    #[default]
    Google,
}

impl fmt::Display for SearchEngine {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchEngine::Ecosia => {
                write!(formatter, "ecosia")
            }
            SearchEngine::Google => {
                write!(formatter, "google")
            }
        }
    }
}

pub struct Runner {
    base_urls: HashMap<SearchEngine, String>,
    config: config::SubdomainsDorkConfig,
}

impl Runner {
    pub fn new(config: config::SubdomainsDorkConfig) -> Self {
        Runner {
            base_urls: HashMap::from([
                (
                    SearchEngine::Ecosia,
                    String::from("https://www.ecosia.org/search?method=index&q={{QUERY}}"),
                ),
                (
                    SearchEngine::Google,
                    String::from("https://www.google.com/search?q={{QUERY}}"),
                ),
            ]),
            config,
        }
    }

    fn name_with_search_engine(&self, search_engine: SearchEngine) -> String {
        format!("{}({})", self.name(), search_engine)
    }

    fn get_domains(
        &self,
        session: &Session,
        domain: String,
        search_engine: SearchEngine,
    ) -> Result<Vec<String>, String> {
        let uri = self
            .base_urls
            .get(&search_engine)
            .unwrap()
            .replace("{{QUERY}}", format!("site%3A{}", domain).as_str());
        if let Ok(response) = session
            .get_http_client()
            .get(uri.clone())
            // https://github.com/benbusby/whoogle-search/issues/1211
            .header(
                USER_AGENT,
                "Lynx/2.9.2 libwww-FM/2.14 SSL-MM/1.4.1 OpenSSL/3.4.0",
            )
            .send()
        {
            let html = response.text().unwrap_or_default();
            let re = Regex::new(&format!(
                r#"\bhttps://([a-zA-Z0-9.-]+\.{})\b"#,
                regex::escape(&domain)
            ))
            .unwrap();
            Ok(re
                .captures_iter(&html)
                .filter_map(|cap| cap.get(1).map(|subdomain| subdomain.as_str().to_string()))
                .collect::<Vec<String>>())
        } else {
            Err(format!("Unable to reach {}", search_engine))
        }
    }

    fn get_emails(
        &self,
        session: &Session,
        domain: String,
        search_engine: SearchEngine,
    ) -> Result<Vec<String>, String> {
        let uri = self
            .base_urls
            .get(&search_engine)
            .unwrap()
            .replace("{{QUERY}}", format!("%22@{}%22", domain).as_str());
        if let Ok(response) = session
            .get_http_client()
            .get(uri.clone())
            // https://github.com/benbusby/whoogle-search/issues/1211
            .header(
                USER_AGENT,
                "Lynx/2.9.2 libwww-FM/2.14 SSL-MM/1.4.1 OpenSSL/3.4.0",
            )
            .send()
        {
            let html = response
                .text()
                .unwrap_or_default()
                .replace(format!("%22@{}%22", domain).as_str(), "");
            let re = Regex::new(&format!(
                r#"\b([a-zA-Z0-9](?:[a-zA-Z0-9.+!%\-/]{{1,64}}|)@{})\b"#,
                regex::escape(&domain)
            ))
            .unwrap();
            Ok(re
                .captures_iter(&html)
                .filter_map(|cap| cap.get(1).map(|email| email.as_str().to_string()))
                .collect::<Vec<String>>())
        } else {
            Err(format!("Unable to reach {}", search_engine))
        }
    }
}

impl Module for Runner {
    fn name(&self) -> String {
        String::from("discovery:subdomains:dork")
    }

    fn description(&self) -> String {
        String::from(
            "Uses search operators and terms on search engines to try gather more information",
        )
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::DiscoveredDomain(String::new())]
    }

    fn execute(&self, session: &Session, context: Context) -> Result<(), String> {
        let domain = match context {
            Context::Domain(domain) => domain,
            _ => {
                return Err("Received wrong context, exiting module".to_string());
            }
        };
        let search_engine = self.config.search_engine.unwrap_or_default();

        match self.get_domains(session, domain.clone(), search_engine) {
            Ok(domains) => {
                for subdomain in domains {
                    if !session
                        .get_state()
                        .has_discovered_domain(subdomain.to_string())
                    {
                        logger::println(
                            self.name_with_search_engine(search_engine),
                            format!("Discovered '{}' as a new subdomain", subdomain),
                        );

                        if let Some(parent) =
                            session.get_database().search(Type::Domain, domain.clone())
                        {
                            let mut new_node = Node::new(Type::Domain, subdomain.to_string());
                            if let Some(ip_addr) = helpers::network::get_ip_addr(&subdomain) {
                                new_node.add_data(
                                    String::from("ip"),
                                    Value::String(ip_addr.to_string()),
                                );
                                if let Some(geoinfo) = helpers::network::geolocate_ip(ip_addr) {
                                    new_node.add_data(String::from("geoinfo"), geoinfo.into())
                                }
                            }
                            parent.connect(new_node);
                        }
                        session.get_state().discover_domain(subdomain.to_string());
                        session.emit(events::Type::DiscoveredDomain(subdomain.to_string()));
                    }
                }
            }
            Err(e) => logger::error(self.name_with_search_engine(search_engine), e),
        }

        match self.get_emails(session, domain.clone(), search_engine) {
            Ok(emails) => {
                for email in emails {
                    if !session.get_state().has_discovered_email(email.to_string()) {
                        logger::println(
                            self.name_with_search_engine(search_engine),
                            format!("Discovered '{}' as a new email", email),
                        );

                        if let Some(parent) =
                            session.get_database().search(Type::Domain, domain.clone())
                        {
                            parent.connect(Node::new(Type::Email, email.to_string()));
                        }
                        session.get_state().discover_email(email.to_string());
                    }
                }
            }
            Err(e) => logger::error(self.name_with_search_engine(search_engine), e),
        }

        Ok(())
    }
}
