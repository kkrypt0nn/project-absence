use std::{fmt, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{event_bus::Event, modules::Module, session::Session};

pub mod wayback_machine;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Runners {
    WaybackMachine,
}

impl fmt::Display for Runners {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Runners::WaybackMachine => {
                write!(formatter, "wayback_machine")
            }
        }
    }
}

pub struct EndpointDiscoveryModule {
    runners: Vec<Box<dyn Module>>,
}

impl EndpointDiscoveryModule {
    pub fn new(runners: Vec<Box<dyn Module>>) -> Self {
        Self { runners }
    }
}

impl Module for EndpointDiscoveryModule {
    fn name(&self) -> String {
        String::from("discovery:endpoint")
    }

    fn description(&self) -> String {
        String::from("Composite module to run multiple endpoint discovery runners")
    }

    fn subscribers(&self) -> Vec<String> {
        self.runners
            .iter()
            .flat_map(|runner| runner.subscribers())
            .collect()
    }

    fn execute(&self, session: Arc<Session>, event: &Event) -> Result<(), String> {
        for runner in &self.runners {
            runner.execute(session.clone(), event)?;
        }
        Ok(())
    }
}
