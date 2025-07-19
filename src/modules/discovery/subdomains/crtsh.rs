use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Mutex;

use reqwest::header::USER_AGENT;

use crate::database::node::{Node, Type};
use crate::modules::{Context, Module};
use crate::session::Session;
use crate::{config, events, flags, helpers, logger};

pub struct Runner {
    config: config::SubdomainsCrtShConfig,
    processed_domains: Mutex<Vec<String>>,
}

impl Runner {
    pub fn new(config: config::SubdomainsCrtShConfig) -> Self {
        Runner {
            config,
            processed_domains: Mutex::new(Vec::new()),
        }
    }

    pub fn process(&self, domain: String) {
        self.processed_domains.lock().unwrap().push(domain)
    }

    pub fn has_processed(&self, domain: String) -> bool {
        self.processed_domains.lock().unwrap().contains(&domain)
    }
}

impl Module for Runner {
    fn name(&self) -> String {
        String::from("discovery:subdomains:crtsh")
    }

    fn description(&self) -> String {
        String::from("This module will perform a passive discovery of new domains by using crt.sh")
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::DiscoveredDomain(String::new())]
    }

    fn execute(&self, session: &Session, context: Context) -> Result<(), String> {
        let domain = match context {
            Context::Domain(domain) => domain,
            _ => {
                return Err("Received wrong context, exiting module".to_string());
            }
        };

        let ignore_expired = self.config.ignore_expired.unwrap_or(false);
        let recent_only = self.config.recent_only.unwrap_or(false);
        if self.has_processed(domain.to_string()) {
            return Ok(());
        }
        self.process(domain.to_string());

        let response = session
            .get_http_client()
            .get(format!("https://crt.sh/?q={}&output=json", domain))
            .header(USER_AGENT, helpers::ua::get_random())
            .send();
        match response {
            Ok(response) => {
                let items: Vec<CrtShItem> = response.json().unwrap_or_default();
                for item in items {
                    let name_values = &item
                        .name_value
                        .split('\n')
                        .map(|x| x.strip_prefix("*.").unwrap_or(x).to_string())
                        .collect::<Vec<String>>();
                    for name_value in name_values {
                        if name_value == &domain.to_string() {
                            continue;
                        }
                        if !session
                            .get_state()
                            .has_discovered_domain(name_value.to_string())
                        {
                            let now = Utc::now();
                            let mut flags = flags::ZERO;

                            // Check if the certificate has expired
                            let has_expired = item.not_after < now;
                            if ignore_expired && has_expired {
                                continue;
                            }
                            if has_expired {
                                flags |= flags::domain::HAS_EXPIRED;
                            }

                            // Check if the certificate has been created within the last 24 hours
                            let is_recent = item.not_before <= now
                                && item.not_before >= now - Duration::try_hours(24).unwrap();
                            if !is_recent && recent_only {
                                continue;
                            }
                            if is_recent {
                                flags |= flags::domain::IS_RECENT;
                            }

                            logger::println(
                                self.name(),
                                format!(
                                    "Discovered '{}' as a new subdomain{}{}",
                                    name_value,
                                    if has_expired {
                                        " $[fg:red]$[effect:bold](Certificate expired, likely inactive)"
                                    } else {
                                        ""
                                    },
                                    if is_recent {
                                        " $[fg:blue]$[effect:bold](Active since less than 24 hours)"
                                    } else {
                                        ""
                                    }
                                ),
                            );

                            if let Some(parent) =
                                session.get_database().search(Type::Domain, domain.clone())
                            {
                                let mut new_node = Node::new(Type::Domain, name_value.clone());
                                new_node.add_flag(flags);
                                if let Some(ip_addr) = helpers::network::get_ip_addr(name_value) {
                                    new_node.add_data(
                                        String::from("ip"),
                                        Value::String(ip_addr.to_string()),
                                    );
                                    if let Some(geoinfo) = helpers::network::geolocate_ip(ip_addr) {
                                        new_node.add_data(String::from("geoinfo"), geoinfo.into())
                                    }
                                }
                                parent.connect(new_node);
                            }
                            session.get_state().discover_domain(name_value.to_string());
                            session.emit(events::Type::DiscoveredDomain(name_value.clone()));
                        }
                    }
                }

                Ok(())
            }
            Err(_) => Err("Failed performing a request to crt.sh (Is it down?)".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrtShItem {
    pub issuer_ca_id: usize,
    pub issuer_name: String,
    pub common_name: Option<String>,
    pub name_value: String,
    pub id: usize,
    #[serde(with = "crt_sh_date_format")]
    pub not_before: DateTime<Utc>,
    #[serde(with = "crt_sh_date_format")]
    pub not_after: DateTime<Utc>,
    pub serial_number: String,
}

mod crt_sh_date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}
