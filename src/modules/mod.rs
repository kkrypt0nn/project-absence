use crate::{event_bus::Event, session::Session};

pub mod dns;
pub mod domain_takeover;
pub mod emails;
pub mod endpoints;
pub mod files;
pub mod infrastructure;
pub mod ready;
pub mod request;
pub mod scripting;
pub mod subdomains;
pub mod technologies;

pub trait Module: Send + Sync {
    fn name(&self) -> String;
    #[allow(dead_code)]
    fn description(&self) -> String;
    fn subscribers(&self) -> Vec<String>;
    fn execute(&self, session: &Session, event: &Event) -> Result<(), String>;
}
