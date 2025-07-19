use crate::events;
use crate::session::Session;

pub mod discovery;
pub mod domain_takeover;
pub mod ready;
pub mod scripting;

#[derive(Clone)]
pub enum Context {
    Domain(String),
    None,
}

pub fn get_context_for_event(event: &events::Type) -> Context {
    match event {
        events::Type::DiscoveredDomain(domain) => Context::Domain(domain.clone()),
        _ => Context::None,
    }
}

pub trait Module: Send + Sync {
    fn name(&self) -> String;
    #[allow(dead_code)]
    fn description(&self) -> String;
    fn subscribers(&self) -> Vec<events::Type>;
    fn execute(&self, session: &Session, context: Context) -> Result<(), String>;
}
