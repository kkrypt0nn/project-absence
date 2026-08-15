use std::{fmt, sync::Arc};

use reqwest::{header::USER_AGENT, tls::TlsInfo};
use x509_parser::parse_x509_certificate;

use crate::{
    event_bus::{self, Event},
    helpers, logger,
    modules::Module,
    session::Session,
};

mod tls;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct HttpResponse {
    pub url: String,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub tls: Option<tls::TlsData>,
}

impl fmt::Debug for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let truncated_body = if self.body.len() > 50 {
            format!("{}...", &self.body[..50])
        } else {
            self.body.clone()
        };

        f.debug_struct("HttpResponse")
            .field("url", &self.url)
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &truncated_body)
            .field("tls", &self.tls)
            .finish()
    }
}

pub struct ModuleRequest;

impl ModuleRequest {
    pub fn new() -> Self {
        ModuleRequest
    }
}

impl Module for ModuleRequest {
    fn name(&self) -> String {
        String::from("request")
    }

    fn description(&self) -> String {
        String::from("Performs a single HTTP request per domain and publishes the response")
    }

    fn subscribers(&self) -> Vec<String> {
        vec![String::from("discovered:domain")]
    }

    fn execute(&self, session: Arc<Session>, event: &Event) -> Result<(), String> {
        let domain = match event {
            Event::DiscoveredDomain(domain) => domain,
            _ => {
                return Err("Received wrong event, exiting module".to_string());
            }
        };

        if session.get_state().has_discovered_domain(domain.clone()) {
            if session.get_state().is_debug_or_verbose() {
                logger::debug(
                    self.name(),
                    format!("Skipping already discovered domain: {}", domain),
                );
            }
            return Ok(());
        }
        session.get_state().discover_domain(domain.clone());

        let url = format!("https://{}", domain);
        let response = match session
            .get_http_client()
            .get(&url)
            .header(USER_AGENT, helpers::ua::get_random())
            .send()
        {
            Ok(r) => r,
            Err(e) => {
                logger::error(self.name(), format!("Failed to request {}: {}", domain, e));
                return Ok(());
            }
        };

        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string().to_lowercase(),
                    v.to_str().unwrap_or_default().to_string().to_lowercase(),
                )
            })
            .collect();
        let tls = response.extensions().get::<TlsInfo>().and_then(|tls_info| {
            tls_info.peer_certificate().and_then(|der| {
                parse_x509_certificate(der)
                    .ok()
                    .map(|(_, cert)| tls::TlsData::from_cert(&cert))
            })
        });
        let body = response.text().unwrap_or_default();

        session.publish(Event::DomainFetched(Box::new(event_bus::DomainFetched {
            domain: domain.clone(),
            response: HttpResponse {
                url,
                status,
                headers,
                body,
                tls,
            },
        })));

        Ok(())
    }
}
