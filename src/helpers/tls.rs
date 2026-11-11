use chrono::Utc;
use der_parser::parse_der;
use reqwest::tls::Version;
use serde::{Deserialize, Serialize};
use x509_certificate::CapturedX509Certificate;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TlsData {
    pub version: String,

    pub subject: String,
    pub issuer: String,

    pub fingerprint: String,

    pub not_before: String,
    pub not_after: String,
    pub expires_in_seconds: i64,

    pub signature_algorithm: String,
    pub public_key_algorithm: String,

    pub san: Vec<String>,
}

impl TlsData {
    pub fn from_cert(cert: &CapturedX509Certificate, version: Option<Version>) -> Self {
        let not_before = cert.validity_not_before();
        let not_after = cert.validity_not_after();
        let expires_in_seconds = (not_after - Utc::now()).num_seconds();

        let fingerprint = cert
            .sha256_fingerprint()
            .map(|digest| {
                digest
                    .as_ref()
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect::<String>()
            })
            .unwrap_or_else(|_| "Unknown".to_string());

        let public_key_algorithm = cert
            .key_algorithm()
            .map(|algorithm| format!("{algorithm:?}"))
            .unwrap_or_else(|| "Unknown".into());

        let signature_algorithm = cert
            .signature_signature_algorithm()
            .map(|algorithm| format!("{algorithm:?}"))
            .unwrap_or_else(|| "Unknown".into());

        Self {
            version: tls_version(version),

            subject: cert
                .subject_common_name()
                .unwrap_or_else(|| format!("{:?}", cert.subject_name())),
            issuer: cert
                .issuer_common_name()
                .unwrap_or_else(|| format!("{:?}", cert.issuer_name())),

            fingerprint,

            not_before: not_before.to_rfc3339(),
            not_after: not_after.to_rfc3339(),
            expires_in_seconds,

            public_key_algorithm,
            signature_algorithm,

            san: extract_san(cert),
        }
    }
}

fn tls_version(version: Option<Version>) -> String {
    match version {
        Some(Version::TLS_1_0) => "TLS 1.0".to_string(),
        Some(Version::TLS_1_1) => "TLS 1.1".to_string(),
        Some(Version::TLS_1_2) => "TLS 1.2".to_string(),
        Some(Version::TLS_1_3) => "TLS 1.3".to_string(),
        Some(_) => "Unknown".to_string(),
        None => "Unknown".to_string(),
    }
}

pub fn extract_san(cert: &CapturedX509Certificate) -> Vec<String> {
    let mut san = vec![];

    for ext in cert.iter_extensions() {
        if ext.id.to_string() != "2.5.29.17" {
            continue;
        }
        let Some(bytes) = ext.value.as_slice() else {
            continue;
        };
        let Ok((_, sequence)) = parse_der(bytes) else {
            continue;
        };
        let Ok(names) = sequence.as_sequence() else {
            continue;
        };

        for name in names {
            if name.header.tag().0 != 2 {
                continue;
            }

            if let der_parser::ber::BerObjectContent::Unknown(any) = &name.content {
                san.push(String::from_utf8_lossy(any.data).into_owned());
            }
        }
    }

    san
}
