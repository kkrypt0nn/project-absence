use std::{
    sync::{
        Arc, RwLock, atomic::{AtomicUsize, Ordering},
    }, thread::sleep, time::Duration,
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

    discovered_domains: RwLock<Vec<String>>,
    discovered_endpoints: RwLock<Vec<String>>,
    discovered_emails: RwLock<Vec<String>>,
    discovered_files: RwLock<Vec<String>>,
}

impl State {
    pub fn new(verbose: bool, debug: bool) -> Self {
        Self {
            active_tasks: Arc::new(AtomicUsize::new(0)),
            semaphore: simple_semaphore::Semaphore::new_available_parallelism()
                .unwrap_or(simple_semaphore::Semaphore::new(8)),
            verbose,
            debug,

            discovered_domains: RwLock::new(vec![]),
            discovered_endpoints: RwLock::new(vec![]),
            discovered_emails: RwLock::new(vec![]),
            discovered_files: RwLock::new(vec![]),
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
                    "tasks={}, memory={}",
                    self.active_tasks_count(),
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
        self.discovered_domains.write().unwrap().push(domain)
    }

    pub fn has_discovered_domain(&self, domain: String) -> bool {
        self.discovered_domains.read().unwrap().contains(&domain)
    }

    pub fn discover_endpoint(&self, endpoint: String) {
        self.discovered_endpoints.write().unwrap().push(endpoint)
    }

    pub fn has_discovered_endpoint(&self, endpoint: String) -> bool {
        self.discovered_endpoints
            .read()
            .unwrap()
            .contains(&endpoint)
    }

    pub fn discover_email(&self, email: String) {
        self.discovered_emails.write().unwrap().push(email)
    }

    pub fn has_discovered_email(&self, email: String) -> bool {
        self.discovered_emails.read().unwrap().contains(&email)
    }

    pub fn discover_file(&self, file: String) {
        self.discovered_files.write().unwrap().push(file)
    }

    pub fn has_discovered_file(&self, file: String) -> bool {
        self.discovered_files.read().unwrap().contains(&file)
    }
}
