use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, RwLock},
};

use crate::modules;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DomainFetched {
    pub domain: String,
    pub response: modules::request::HttpResponse,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Ready,

    /// A domain was discovered for the first time
    DiscoveredDomain(String),

    /// A domain has been requested
    DomainFetched(Box<DomainFetched>),

    FinishedTask,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::Ready => {
                write!(f, "ready")
            }
            Event::DiscoveredDomain(_) => {
                write!(f, "discovered:domain")
            }
            Event::DomainFetched(_) => {
                write!(f, "domain:fetched")
            }
            Event::FinishedTask => write!(f, "finished:task"),
        }
    }
}

type CallbackFn = Arc<dyn Fn(&Event) + Send + Sync>;

#[derive(Clone)]
pub struct EventBus {
    subscribers: Arc<RwLock<HashMap<String, Vec<CallbackFn>>>>,
}

impl EventBus {
    pub fn new() -> EventBus {
        EventBus {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn subscribe<F>(&self, event_name: &str, callback: F)
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        let mut subs = self.subscribers.write().unwrap();
        subs.entry(event_name.to_string())
            .or_default()
            .push(Arc::new(callback));
    }

    pub fn publish(&self, event: &Event) {
        let subs = self.subscribers.read().unwrap();
        if let Some(callbacks) = subs.get(&event.to_string()) {
            let callbacks = callbacks.clone();
            drop(subs);

            for cb in callbacks {
                cb(event);
            }
        }
    }
}
