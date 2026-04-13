use std::{collections::HashMap, env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    logger,
    modules::discovery::{emails, endpoints, subdomains},
};

const DEFAULT_CONFIG: &str = r#"[domain_takeover]
enabled = true

[dns]
enabled = true
record_types = ["a", "aaaa", "caa", "mx", "soa", "txt"]

[dns.interesting_keywords]
mx = ["protonmail.ch"]

[emails]
enabled_runners = ["dork"]

[emails.dork]
search_engine = "ecosia"

[endpoints]
enabled_runners = ["wayback_machine"]

[endpoints.wayback_machine]
timeout = 30

[infrastructure]
enabled = true

[subdomains]
enabled_runners = ["dork", "crtsh"]

[subdomains.dork]
search_engine = "ecosia"

[subdomains.crtsh]
ignore_expired = false
recent_only = false

[technologies]
enabled = true
"#;

pub fn create_file_if_not_existing() {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| String::from(""));
    let path = PathBuf::from(format!("{}/.absence/config.toml", home_dir));
    if !path.exists()
        && let Some(parent) = path.parent()
    {
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

/// The config.toml file structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub domain_takeover: Option<DomainTakeoverConfig>,
    pub dns: Option<DnsConfig>,
    pub endpoints: Option<EndpointsConfig>,
    pub emails: Option<EmailsConfig>,
    pub infrastructure: Option<InfrastructureConfig>,
    pub subdomains: Option<SubdomainsConfig>,
    pub technologies: Option<TechnologiesConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainTakeoverConfig {
    /// Whether the module is enabled
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnsConfig {
    /// Whether the module is enabled
    pub enabled: bool,
    /// The type of DNS records to request for
    pub record_types: Vec<String>,
    /// The interesting keywords per record type to flag
    pub interesting_keywords: DnsInterestingKeywordsConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnsInterestingKeywordsConfig {
    #[serde(flatten)]
    pub records: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfrastructureConfig {
    /// Whether the module is enabled
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailsConfig {
    /// List of enabled runners
    pub enabled_runners: Option<Vec<emails::Runners>>,
    // Configuration for the dork runner
    pub dork: Option<EmailsDorkConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EmailsDorkConfig {
    /// The search engine to use
    pub search_engine: Option<emails::dork::SearchEngine>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndpointsConfig {
    /// List of enabled runners
    pub enabled_runners: Option<Vec<endpoints::Runners>>,
    // Configuration for the Wayback Machine runner
    pub wayback_machine: Option<EndpointsWaybackMachineConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EndpointsWaybackMachineConfig {
    /// The timeout, in seconds, to use when requesting the endpoint, default is 30
    pub timeout: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubdomainsConfig {
    /// List of enabled runners
    pub enabled_runners: Option<Vec<subdomains::Runners>>,
    // Configuration for the dork runner
    pub dork: Option<SubdomainsDorkConfig>,
    // Configuration for the crt.sh runner
    pub crtsh: Option<SubdomainsCrtShConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SubdomainsDorkConfig {
    /// The search engine to use
    pub search_engine: Option<subdomains::dork::SearchEngine>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SubdomainsCrtShConfig {
    /// Ignore expired certificates
    pub ignore_expired: Option<bool>,
    /// Only care about the recently (24 hours) created certificates
    pub recent_only: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TechnologiesConfig {
    /// Whether the module is enabled
    pub enabled: bool,
}
