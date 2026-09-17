use std::{collections::HashMap, fmt, sync::Arc};

use regex::Regex;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

use crate::{database::node::{Node, Type}, event_bus::Event, modules::Module, session::Session, config, logger};

#[derive(
    Copy, Clone, Debug, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash,
)]
#[serde(rename_all = "lowercase")]
pub enum SearchEngine {
    #[default]
    Brave,
    Ecosia,
    Google,
}

impl fmt::Display for SearchEngine {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Brave => {
                write!(formatter, "brave")
            }
            Self::Ecosia => {
                write!(formatter, "ecosia")
            }
            Self::Google => {
                write!(formatter, "google")
            }
        }
    }
}

pub struct Runner {
    base_urls: HashMap<SearchEngine, String>,
    config: config::EmailsDorkConfig,
}

impl Runner {
    pub fn new(config: config::EmailsDorkConfig) -> Self {
        Self {
            base_urls: HashMap::from([
                (
                    SearchEngine::Brave,
                    String::from("https://search.brave.com/search?q={{QUERY}}"),
                ),
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
        format!("{}({search_engine})", self.name())
    }

    fn get_emails(
        &self,
        session: Arc<Session>,
        domain: String,
        search_engine: SearchEngine,
    ) -> Result<Vec<String>, String> {
        let uri = self
            .base_urls
            .get(&search_engine)
            .unwrap()
            .replace("{{QUERY}}", format!("\"%40{domain}\"").as_str());
        if let Ok(response) = session
            .get_http_client()
            .get(uri)
            // https://github.com/benbusby/whoogle-search/issues/1211
            .header(
                USER_AGENT,
                "Lynx/2.9.2 libwww-FM/2.14 SSL-MM/1.4.1 OpenSSL/3.4.0",
            )
            .send()
        {
            let html = response.text().unwrap_or_default();
            // https://stackoverflow.com/questions/201323/how-can-i-validate-an-email-address-using-a-regular-expression
            let re = Regex::new(&format!(
                r#"(?:[A-Za-z0-9!#$%&'*+\x2f=?^_`\x7b-\x7d~\x2d]+(?:\.[A-Za-z0-9!#$%&'*+\x2f=?^_`\x7b-\x7d~\x2d]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@{}"#,
                regex::escape(&domain)
            ))
            .unwrap();
            Ok(re
                .captures_iter(&html)
                .filter_map(|cap| cap.get(0).map(|email| email.as_str().to_string()))
                .collect::<Vec<String>>())
        } else {
            Err(format!("Unable to reach {search_engine}"))
        }
    }
}

impl Module for Runner {
    fn name(&self) -> &'static str {
        "discovery:emails:dork"
    }

    fn description(&self) -> String {
        String::from(
            "Uses search operators and terms on search engines to try gather more information",
        )
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("discovered:domain")]
    }

    fn execute(&self, session: Arc<Session>, event: &Event) -> Result<(), String> {
        let Event::DiscoveredDomain(domain) = event else {
            return Err("Received wrong event, exiting module".to_string());
        };
        let search_engine = self.config.search_engine.unwrap_or_default();

        match self.get_emails(session.clone(), domain.clone(), search_engine) {
            Ok(emails) => {
                for email in emails {
                    if !session.get_state().has_discovered_email(email.to_string()) {
                        logger::println(
                            self.name_with_search_engine(search_engine),
                            format!("Discovered '{email}' as a new email"),
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
