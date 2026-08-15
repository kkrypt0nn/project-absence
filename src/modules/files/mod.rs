use std::{fmt, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{event_bus::Event, modules::Module, session::Session};

pub mod dork;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Runners {
    Dork,
}

impl fmt::Display for Runners {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Runners::Dork => {
                write!(formatter, "dork")
            }
        }
    }
}

pub struct FileDiscoveryModule {
    runners: Vec<Box<dyn Module>>,
}

impl FileDiscoveryModule {
    pub fn new(runners: Vec<Box<dyn Module>>) -> Self {
        Self { runners }
    }
}

impl Module for FileDiscoveryModule {
    fn name(&self) -> String {
        String::from("discovery:files")
    }

    fn description(&self) -> String {
        String::from("Composite module to run multiple file discovery runners")
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
