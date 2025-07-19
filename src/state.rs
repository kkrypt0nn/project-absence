use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread::sleep,
    time::Duration,
};

use human_bytes::human_bytes;
use memory_stats::memory_stats;
use simple_semaphore::Permit;

use crate::logger;

pub struct State {
    active_tasks: Arc<AtomicUsize>,
    semaphore: Arc<simple_semaphore::Semaphore>,
    verbose: bool,
    debug: bool,

    discovered_domains: Mutex<Vec<String>>,
    discovered_emails: Mutex<Vec<String>>,
}

impl State {
    pub fn new(verbose: bool, debug: bool) -> Self {
        State {
            active_tasks: Arc::new(AtomicUsize::new(0)),
            semaphore: simple_semaphore::Semaphore::default(),
            verbose,
            debug,

            discovered_domains: Mutex::new(Vec::new()),
            discovered_emails: Mutex::new(Vec::new()),
        }
    }

    pub fn actively_report(&self) {
        let dur = Duration::from_secs(1);
        loop {
            sleep(dur);
            let usage = if let Some(usage) = memory_stats() {
                usage.physical_mem
            } else {
                logger::error("state", "Couldn't get the current memory usage");
                0
            };
            logger::info(
                "state",
                format!(
                    "tasks={}, permits={}, memory={}",
                    self.active_tasks_count(),
                    self.semaphore.available_permits(),
                    human_bytes(usage as f64),
                ),
            );
        }
    }

    pub fn increment_tasks(&self) {
        self.active_tasks.fetch_add(1, Ordering::SeqCst);
        if self.is_debug() {
            logger::debug("state", "The running task counter has incremented");
        }
    }

    pub fn decrement_tasks(&self) {
        self.active_tasks.fetch_sub(1, Ordering::SeqCst);
        if self.is_debug() {
            logger::debug("state", "The running task counter has decremented");
        }
    }

    pub fn active_tasks_count(&self) -> usize {
        self.active_tasks.load(Ordering::SeqCst)
    }

    pub fn get_semaphore_permit(&self) -> Permit {
        self.semaphore.acquire()
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn is_debug_or_verbose(&self) -> bool {
        self.is_debug() || self.is_verbose()
    }

    pub fn discover_domain(&self, domain: String) {
        self.discovered_domains.lock().unwrap().push(domain)
    }

    pub fn has_discovered_domain(&self, domain: String) -> bool {
        self.discovered_domains.lock().unwrap().contains(&domain)
    }

    pub fn discover_email(&self, email: String) {
        self.discovered_emails.lock().unwrap().push(email)
    }

    pub fn has_discovered_email(&self, email: String) -> bool {
        self.discovered_emails.lock().unwrap().contains(&email)
    }
}
