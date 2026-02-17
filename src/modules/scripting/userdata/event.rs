use mlua::{UserData, UserDataMethods};

use crate::event_bus::Event;

pub struct LuaEvent {
    event: Event,
}

impl LuaEvent {
    pub fn new(event: &Event) -> Self {
        Self {
            event: event.clone(),
        }
    }
}

impl UserData for LuaEvent {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| Ok(this.event.to_string()));

        methods.add_method("domain", |_, this, ()| match &this.event {
            Event::DiscoveredDomain(domain) => Ok(Some(domain.clone())),
            Event::DomainFetched(domain_fetched) => Ok(Some(domain_fetched.domain.clone())),
            _ => Ok(None),
        });

        methods.add_method("is", |_, this, name: String| {
            Ok(this.event.to_string() == name)
        });
    }
}
