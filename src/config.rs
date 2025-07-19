use std::{env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    logger,
    modules::discovery::subdomains::{self, dork},
};

const DEFAULT_CONFIG: &str = r#"[domain_takeover]
enabled = true

[subdomains]
enabled_runners = ["dork", "crtsh"]

[subdomains.dork]
search_engine = "google"

[subdomains.crtsh]
ignore_expired = false
recent_only = false
"#;

pub fn create_file_if_not_existing() {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| String::from(""));
    let path = PathBuf::from(format!("{}/.absence/config.toml", home_dir));
    if !path.exists() {
        if let Some(parent) = path.parent() {
            if fs::create_dir_all(parent).is_err() {
                logger::error(
                    "setup",
                    "Failed creating the directories for the default config file",
                );
            }
            if fs::write(path, DEFAULT_CONFIG).is_err() {
                logger::error(
                    "setup",
                    "Failed writing the default content of the config file",
                );
            }
        }
    }
}

/// The config.toml file structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub domain_takeover: Option<DomainTakeoverConfig>,
    pub subdomains: Option<SubdomainsConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainTakeoverConfig {
    /// Whether the module is enabled
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubdomainsConfig {
    /// List of enabled runners like "dork", "crtsh", etc.
    pub enabled_runners: Option<Vec<subdomains::Runners>>,
    // Configuration for the dork runner
    pub dork: Option<SubdomainsDorkConfig>,
    // Configuration for the crt.sh runner
    pub crtsh: Option<SubdomainsCrtShConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubdomainsDorkConfig {
    /// The search engine to use
    /// TODO: Allow multiple, default is all
    pub search_engine: Option<dork::SearchEngine>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubdomainsCrtShConfig {
    /// Ignore expired certificates
    pub ignore_expired: Option<bool>,
    /// Only care about the recently (24 hours) created certificates
    pub recent_only: Option<bool>,
}
