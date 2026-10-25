use std::{collections::HashMap, sync::Arc};

use crate::{
    database::node::{NodeMetadata, Type},
    event_bus::Event,
    logger,
    modules::Module,
    session::Session,
};

pub struct ModuleTechnologies {
    signatures: HashMap<&'static str, Vec<&'static str>>,
}

impl Default for ModuleTechnologies {
    fn default() -> Self {
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

        Self { signatures }
    }
}

impl Module for ModuleTechnologies {
    fn name(&self) -> &'static str {
        "technologies"
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
        let Event::DomainFetched(fetched_data) = event else {
            return Err("Received wrong event, exiting module".to_string());
        };
        let domain = &fetched_data.domain;
        let body = &fetched_data.response.body;

        let mut technologies_found: Vec<String> = vec![];
        for (tech, keywords) in &self.signatures {
            if keywords.iter().any(|keyword| body.contains(keyword)) {
                technologies_found.push(tech.to_string());
            }
        }

        if let Some(parent) = session
            .get_database()
            .search(Type::Domain, domain.to_string())
        {
            if let NodeMetadata::Subdomain(metadata) = &mut parent.metadata {
                metadata.technologies = Some(technologies_found.clone());
            }
            logger::println(
                self.name(),
                format!("Technologies found: {}", technologies_found.join(", ")),
            );
        }

        Ok(())
    }
}
