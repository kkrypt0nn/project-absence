use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::vec;

use domain::base::{Name, Rtype};
use domain::rdata::AllRecordData;
use domain::resolv::StubResolver;
use serde::Serialize;

use crate::event_bus::Event;
use crate::modules::Module;
use crate::session::Session;
use crate::{config, flags, logger};

#[derive(Serialize)]
struct DnsRecordEntry {
    value: String,
    flags: usize,
}

pub struct ModuleDns {
    config: config::DnsConfig,
}

impl ModuleDns {
    pub fn new(config: config::DnsConfig) -> Self {
        ModuleDns { config }
    }

    fn name_with_record_type(&self, record_type: Rtype) -> String {
        format!("{}({})", self.name(), record_type)
    }
}

impl Module for ModuleDns {
    fn name(&self) -> String {
        String::from("dns")
    }

    fn description(&self) -> String {
        String::from(
            "This module gets data about the DNS entries of the domain, and flags certain keywords as interesting",
        )
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("domain:fetched")]
    }

    fn execute(&self, _session: Arc<Session>, event: &Event) -> Result<(), String> {
        let fetched_data = match event {
            Event::DomainFetched(fetched_data) => fetched_data,
            _ => return Err("Received wrong event, exiting module".to_string()),
        };
        let domain = &fetched_data.domain;
        let name =
            Name::<Vec<u8>>::from_str(domain).map_err(|_| format!("Invalid domain: {}", domain))?;

        let mut dns_data: HashMap<String, Vec<DnsRecordEntry>> = HashMap::new();

        for record_type in &self.config.record_types {
            let rtype = match Rtype::from_str(record_type) {
                Ok(r) => r,
                Err(_) => {
                    logger::error(self.name(), format!("Invalid record type: {}", record_type));
                    continue;
                }
            };

            // Cloning a Vec<u8> is probably cheaper than parsing all the time (?)
            let name = name.clone();
            let resolver =
                StubResolver::run(move |stub| async move { stub.query((name, rtype)).await });
            let result = match resolver {
                Ok(res) => res,
                Err(e) => {
                    logger::error(
                        self.name_with_record_type(rtype),
                        format!("Query failed: {}", e),
                    );
                    continue;
                }
            };

            let records = match result.answer() {
                Ok(ans) => ans.limit_to::<AllRecordData<_, _>>(),
                Err(_) => continue,
            };

            for record in records {
                match record {
                    Ok(rec) => {
                        let data = rec.data().to_string();
                        let mut flags = 0;
                        if let Some(keywords) =
                            self.config.interesting_keywords.records.get(record_type)
                            && keywords.iter().any(|k| data.contains(k))
                        {
                            flags |= flags::dns::IS_INTERESTING;
                            logger::println(
                                self.name_with_record_type(rtype),
                                format!("[INTERESTING] {}", data),
                            );
                        }
                        dns_data
                            .entry(record_type.to_string())
                            .or_default()
                            .push(DnsRecordEntry { value: data, flags });
                    }
                    Err(e) => {
                        logger::error(
                            self.name_with_record_type(rtype),
                            format!("Failed to parse record: {}", e),
                        );
                    }
                }
            }
        }

        if let Some(parent) = _session
            .get_database()
            .search(crate::database::node::Type::Domain, domain.to_string())
        {
            parent.add_data("dns".to_string(), serde_json::to_value(dns_data).unwrap());
            logger::println(self.name(), format!("Stored DNS data for {}", domain));
        }

        Ok(())
    }
}
