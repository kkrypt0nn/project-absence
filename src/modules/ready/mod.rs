use std::sync::Arc;

use crate::{event_bus::Event, logger, modules::Module, session::Session};

pub struct ModuleReady;

impl Module for ModuleReady {
    fn name(&self) -> &'static str {
        "ready"
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
