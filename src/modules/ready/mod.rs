use std::sync::Arc;

use crate::event_bus::Event;
use crate::logger;
use crate::modules::Module;
use crate::session::Session;

pub struct ModuleReady {}

impl ModuleReady {
    pub fn new() -> Self {
        ModuleReady {}
    }
}

impl Module for ModuleReady {
    fn name(&self) -> String {
        String::from("ready")
    }

    fn description(&self) -> String {
        String::from(
            "This module is responsible to know when Project Absence is ready and will start to do the work",
        )
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("ready")]
    }

    fn execute(&self, _: Arc<Session>, _: &Event) -> Result<(), String> {
        logger::println(
            self.name(),
            "Project Absence is now ready and will start doing its magic!",
        );

        Ok(())
    }
}
