use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{event_bus::Event, modules::Module, session::Session};

pub mod crtsh;
pub mod dork;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Runners {
    Crtsh,
    Dork,
}

impl fmt::Display for Runners {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Runners::Crtsh => {
                write!(formatter, "crtsh")
            }
            Runners::Dork => {
                write!(formatter, "dork")
            }
        }
    }
}

pub struct SubdomainDiscoveryModule {
    runners: Vec<Box<dyn Module>>,
}

impl SubdomainDiscoveryModule {
    pub fn new(runners: Vec<Box<dyn Module>>) -> Self {
        Self { runners }
    }
}

impl Module for SubdomainDiscoveryModule {
    fn name(&self) -> String {
        String::from("discovery:subdomains")
    }

    fn description(&self) -> String {
        String::from("Composite module to run multiple subdomain discovery runners")
    }

    fn subscribers(&self) -> Vec<String> {
        self.runners
            .iter()
            .flat_map(|runner| runner.subscribers())
            .collect()
    }

    fn execute(&self, session: &Session, event: &Event) -> Result<(), String> {
        for runner in &self.runners {
            runner.execute(session, event)?;
        }
        Ok(())
    }
}
