use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use x509_parser::prelude::{GeneralName, X509Certificate};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TlsData {
    pub serial: String,
    pub subject: String,
    pub issuer: String,
    pub not_before: String,
    pub not_after: String,
    pub expires_in_seconds: i64,
    pub key_type: String,
    pub san: Vec<String>,
}

impl TlsData {
    pub fn from_cert(cert: &X509Certificate) -> Self {
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

        Self {
            serial: cert.raw_serial_as_string(),
            subject: cert.subject().to_string(),
            issuer: cert.issuer().to_string(),
            not_before: not_before.to_string(),
            not_after: not_after.to_string(),
            expires_in_seconds,
            key_type: key_type(cert),
            san,
        }
    }
}

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
