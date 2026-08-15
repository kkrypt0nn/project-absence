use std::collections::HashMap;
use std::sync::Arc;
use std::vec;

use crate::database::node::Type;
use crate::event_bus::Event;
use crate::logger;
use crate::modules::Module;
use crate::session::Session;

pub struct ModuleTechnologies {
    signatures: HashMap<&'static str, Vec<&'static str>>,
}

impl ModuleTechnologies {
    pub fn new() -> Self {
        let mut signatures: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

        signatures.insert("astro", vec!["content=\"Astro v\"", "data-astro-cid-"]);
        // Find some for React other than id="root" or id="app"
        signatures.insert("react", vec![]);
        signatures.insert("nextjs", vec!["/_next/static/"]);
        signatures.insert("vue", vec!["data-v-"]);
        signatures.insert("nuxt", vec!["id=\"__nuxt\"", "window.__NUXT__"]);
        signatures.insert(
            "angular",
            vec!["ng-version", "_ngcontent-ng-", "_nghost-ng-"],
        );

        ModuleTechnologies { signatures }
    }
}

impl Module for ModuleTechnologies {
    fn name(&self) -> String {
        String::from("technologies")
    }

    fn description(&self) -> String {
        String::from(
            "This module gets data about the technology stack used by looking at the source code of the page",
        )
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("domain:fetched")]
    }

    fn execute(&self, session: Arc<Session>, event: &Event) -> Result<(), String> {
        let fetched_data = match event {
            Event::DomainFetched(fetched_data) => fetched_data,
            _ => return Err("Received wrong event, exiting module".to_string()),
        };
        let domain = &fetched_data.domain;
        let body = &fetched_data.response.body;

        let mut technologies_found: Vec<&str> = Vec::new();
        for (tech, keywords) in &self.signatures {
            if keywords.iter().any(|keyword| body.contains(keyword)) {
                technologies_found.push(tech);
            }
        }

        if let Some(parent) = session
            .get_database()
            .search(Type::Domain, domain.to_string())
        {
            parent.add_data(
                "technologies".to_string(),
                serde_json::to_value(&technologies_found).unwrap(),
            );
            logger::println(
                self.name(),
                format!(
                    "Technologies found: {}",
                    technologies_found.join(", ").as_str()
                ),
            );
        }

        Ok(())
    }
}
