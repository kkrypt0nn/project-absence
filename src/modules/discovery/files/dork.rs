use std::collections::HashMap;
use std::fmt;

use regex::Regex;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

use crate::database::node::{Node, Type};
use crate::event_bus::Event;
use crate::modules::Module;
use crate::session::Session;
use crate::{config, logger};

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
            SearchEngine::Brave => {
                write!(formatter, "brave")
            }
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
    config: config::FilesDorkConfig,
}

impl Runner {
    pub fn new(config: config::FilesDorkConfig) -> Self {
        Runner {
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
        format!("{}({})", self.name(), search_engine)
    }

    fn get_files(
        &self,
        session: &Session,
        domain: String,
        search_engine: SearchEngine,
    ) -> Result<Vec<String>, String> {
        let mut results = Vec::new();

        for filetype in &self.config.file_types {
            let query = format!("filetype:{} site:{}", filetype, domain);
            let uri = self
                .base_urls
                .get(&search_engine)
                .unwrap()
                .replace("{{QUERY}}", &query);
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
                println!("{}", html);
                let re = Regex::new(&format!(
                    r#"href="(https?://[^"]*{}[^"]*\.{}[^"]*)""#,
                    regex::escape(&domain),
                    regex::escape(filetype),
                ))
                .unwrap();
                results.extend(
                    re.captures_iter(&html)
                        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string())),
                );
            } else {
                return Err(format!("Unable to reach {}", search_engine));
            }
        }

        Ok(results)
    }
}

impl Module for Runner {
    fn name(&self) -> String {
        String::from("discovery:files:dork")
    }

    fn description(&self) -> String {
        String::from(
            "Uses search operators and terms on search engines to try gather more information",
        )
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("discovered:domain")]
    }

    fn execute(&self, session: &Session, event: &Event) -> Result<(), String> {
        let domain = match event {
            Event::DiscoveredDomain(domain) => domain,
            _ => return Err("Received wrong event, exiting module".to_string()),
        };
        let search_engine = self.config.search_engine.unwrap_or_default();

        match self.get_files(session, domain.clone(), search_engine) {
            Ok(files) => {
                for file in files {
                    if !session.get_state().has_discovered_file(file.to_string()) {
                        logger::println(
                            self.name_with_search_engine(search_engine),
                            format!("Discovered '{}' as a new file", file),
                        );

                        if let Some(parent) =
                            session.get_database().search(Type::Domain, domain.clone())
                        {
                            parent.connect(Node::new(Type::File, file.to_string()));
                        }
                        session.get_state().discover_file(file.to_string());
                    }
                }
            }
            Err(e) => logger::error(self.name_with_search_engine(search_engine), e),
        }

        Ok(())
    }
}
