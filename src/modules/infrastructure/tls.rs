use chrono::{TimeZone, Utc};
use reqwest::{blocking::Response, tls::TlsInfo};
use serde_json::json;
use x509_parser::{
    parse_x509_certificate,
    prelude::{GeneralName, X509Certificate},
};

// https://oidref.com/1.2.840
// https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-crypt_algorithm_identifier
fn key_type(cert: &X509Certificate) -> String {
    let oid = &cert.public_key().algorithm.algorithm;
    match oid.to_string().as_str() {
        "1.2.840.113549.1.1.1" => "RSA".to_string(),
        "1.2.840.10045.2.1" => "EC".to_string(),
        "1.2.840.10040.4.1" => "DSA".to_string(),
        _ => format!("Unknown({})", oid),
    }
}

pub fn extract(response: &Response) -> Option<serde_json::Value> {
    let tls = response.extensions().get::<TlsInfo>()?;
    let der = tls.peer_certificate()?;
    let (_, cert) = parse_x509_certificate(der).ok()?;

    let not_before = Utc
        .timestamp_opt(cert.validity.not_before.timestamp(), 0)
        .unwrap();
    let not_after = Utc
        .timestamp_opt(cert.validity.not_after.timestamp(), 0)
        .unwrap();
    let expires_in_seconds = (not_after - Utc::now()).num_seconds();
    let san = match cert.subject_alternative_name() {
        Ok(Some(extension)) => extension
            .value
            .general_names
            .iter()
            .map(|n| match n {
                GeneralName::DNSName(s) => s.to_string(),
                _ => n.to_string(),
            })
            .collect(),
        _ => Vec::new(),
    };

    Some(json!({
        "serial": cert.raw_serial_as_string(),
        "subject": cert.subject().to_string(),
        "issuer": cert.issuer().to_string(),
        "not_before": not_before.to_string(),
        "not_after": not_after.to_string(),
        "expires_in_seconds": expires_in_seconds,
        "key_type": key_type(&cert),
        "san": san,
    }))
}
