use std::{
    collections::HashMap,
    fmt::{self, Write},
};

use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;

use crate::{
    flags,
    helpers::{network::GeoInfo, tls::TlsData},
    modules::dns::DnsRecordEntry,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Domain,
    // Maybe this one will be renamed to WaybackMachineEndpoint or add a flag?
    Endpoint,
    Email,
    File,
}

impl fmt::Display for Type {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Domain => {
                write!(formatter, "domain")
            }
            Self::Endpoint => {
                write!(formatter, "endpoint")
            }
            Self::Email => {
                write!(formatter, "email")
            }
            Self::File => {
                write!(formatter, "file")
            }
        }
    }
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DomainData {
    pub is_root: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SubdomainData {
    pub dns: Option<HashMap<String, Vec<DnsRecordEntry>>>,
    pub possible_takeover: Option<bool>,
    pub takeover_platform: Option<String>,
    pub tls: Option<TlsData>,
    pub cloud_provider: Option<String>,
    pub interesting_headers: Option<HashMap<String, String>>,
    pub security_headers: Option<HashMap<String, String>>,
    pub ip: Option<String>,
    pub geoinfo: Option<GeoInfo>,
    pub technologies: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", content = "metadata", rename_all = "lowercase")]
pub enum NodeMetadata {
    Domain(DomainData),
    Subdomain(SubdomainData),
    Endpoint,
    Email,
    File,
}

#[derive(Clone, Debug, Serialize)]
pub struct Node {
    pub r#type: Type,
    value: String,
    connections: Vec<Self>,
    pub flags: usize,
    pub metadata: NodeMetadata,
    pub data: HashMap<String, Value>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node {{ type: {}, value: {}, connections: {}, data: {:?} }}",
            self.r#type,
            self.value,
            self.get_connections().len(),
            self.data
        )
    }
}

impl Node {
    pub fn new(r#type: Type, value: String) -> Self {
        let metadata = match r#type {
            Type::Domain => NodeMetadata::Subdomain(SubdomainData::default()),
            Type::Endpoint => NodeMetadata::Endpoint,
            Type::Email => NodeMetadata::Email,
            Type::File => NodeMetadata::File,
        };
        Self {
            r#type,
            value,
            connections: vec![],
            flags: flags::ZERO,
            metadata,
            data: HashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn connect(&mut self, node: Self) {
        self.connections.push(node);
    }

    #[allow(unused)]
    pub fn add(&mut self, r#type: Type, value: String) {
        self.connect(Self::new(r#type, value));
    }

    pub fn add_data(&mut self, key: String, value: Value) {
        self.data.insert(key, value);
    }

    pub fn get_data(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    #[allow(dead_code)]
    pub fn edit_data(&mut self, key: String, new_value: Value) {
        self.data.entry(key).and_modify(|e| *e = new_value);
    }

    pub fn add_flag(&mut self, flag: usize) {
        self.flags |= flag;
    }

    #[allow(dead_code)]
    pub fn has_flag(&self, flag: usize) -> bool {
        flags::contains(self.flags, flag)
    }

    #[allow(dead_code)]
    pub fn get_or_init_map(&mut self, key: &str) -> serde_json::Map<String, Value> {
        if !self.data.contains_key(key) {
            self.data
                .insert(key.to_string(), Value::Object(serde_json::Map::new()));
        }
        self.get_data(key)
            .and_then(|value| value.as_object().cloned())
            .expect("JSON object should exist")
    }

    pub fn get_connections(&self) -> Vec<Self> {
        self.connections.clone()
    }

    pub fn find(&mut self, r#type: &Type, value: &str) -> Option<&mut Self> {
        if &self.r#type == r#type && self.value == value {
            return Some(self);
        }

        for connection in &mut self.connections {
            if let Some(found) = connection.find(r#type, value) {
                return Some(found);
            }
        }

        None
    }

    pub fn to_markdown(&self) -> String {
        let flags = if self.flags != flags::ZERO {
            let mut result = String::from("#### Flags\n");
            write!(
                &mut result,
                "\n- `IS_RECENT` => {}\n- `HAS_EXPIRED` => {}\n- `POSSIBLE_TAKEOVER` => {}",
                flags::contains_to_markdown(self.flags, flags::domain::IS_RECENT),
                flags::contains_to_markdown(self.flags, flags::domain::HAS_EXPIRED),
                if flags::contains(self.flags, flags::domain::POSSIBLE_TAKEOVER) {
                    format!(
                        "✅ (Platform: `{}`)",
                        match &self.metadata {
                            NodeMetadata::Subdomain(metadata) =>
                                metadata.takeover_platform.as_deref().unwrap_or("Not set"),
                            _ => "Not set",
                        }
                    )
                } else {
                    "❌".to_string()
                }
            )
            .unwrap();
            Some(result)
        } else {
            None
        };

        let mut data_markdown = String::new();
        for (key, value) in &self.data {
            write!(
                &mut data_markdown,
                "#### {key}\n\n{}\n\n",
                match value {
                    Value::Object(_) | Value::Array(_) => {
                        format!(
                            "```json\n{}\n```",
                            serde_json::to_string_pretty(value).unwrap_or_default()
                        )
                    }
                    _ => value.to_string(),
                }
            )
            .unwrap();
        }

        let mut metadata_markdown = String::new();

        match &self.metadata {
            NodeMetadata::Domain(metadata) => {
                write!(
                    &mut metadata_markdown,
                    "#### Is root\n\n{}\n\n",
                    metadata.is_root,
                )
                .unwrap();
            }
            NodeMetadata::Subdomain(metadata) => {
                if let Some(dns) = &metadata.dns {
                    write!(
                        &mut metadata_markdown,
                        "#### DNS\n\n{}\n\n",
                        dns.iter()
                            .map(|(record_type, records)| {
                                format!(
                                    "**{record_type}**\n{}",
                                    records
                                        .iter()
                                        .map(|record| format!(
                                            "- `{}` (flags: {})",
                                            record.value, record.flags
                                        ))
                                        .collect::<Vec<_>>()
                                        .join("\n")
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n\n")
                    )
                    .unwrap();
                }

                if let Some(possible_takeover) = metadata.possible_takeover {
                    write!(
                        &mut metadata_markdown,
                        "#### Possible takeover\n\n{}\n\n",
                        if possible_takeover {
                            "✅ Yes"
                        } else {
                            "❌ No"
                        }
                    )
                    .unwrap();
                }

                if let Some(tls) = &metadata.tls {
                    write!(
                        &mut metadata_markdown,
                        "#### TLS\n\n```json\n{}\n```\n\n",
                        serde_json::to_string_pretty(tls).unwrap_or_default()
                    )
                    .unwrap();
                }

                if let Some(cloud_provider) = &metadata.cloud_provider {
                    write!(
                        &mut metadata_markdown,
                        "#### Cloud provider\n\n{cloud_provider}\n\n"
                    )
                    .unwrap();
                }

                if let Some(headers) = &metadata.interesting_headers {
                    write!(
                        &mut metadata_markdown,
                        "#### Interesting headers\n\n{}\n\n",
                        headers
                            .iter()
                            .map(|(key, value)| format!("- `{key}: {value}`"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                    .unwrap();
                }

                if let Some(headers) = &metadata.security_headers {
                    write!(
                        &mut metadata_markdown,
                        "#### Security headers\n\n{}\n\n",
                        headers
                            .iter()
                            .map(|(key, value)| format!("- `{key}: {value}`"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                    .unwrap();
                }

                if let Some(ip) = &metadata.ip {
                    write!(&mut metadata_markdown, "#### IP\n\n{ip}\n\n").unwrap();
                }

                if let Some(geoinfo) = &metadata.geoinfo {
                    write!(
                        &mut metadata_markdown,
                        "#### Geolocation\n\n- **City:** {}\n- **Country:** {}\n\n",
                        geoinfo.city.as_deref().unwrap_or("Unknown"),
                        geoinfo.country.as_deref().unwrap_or("Unknown")
                    )
                    .unwrap();
                }

                if let Some(technologies) = &metadata.technologies {
                    write!(
                        &mut metadata_markdown,
                        "#### Technologies\n\n{}\n\n",
                        technologies
                            .iter()
                            .map(|technology| format!("- {technology}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                    .unwrap();
                }
            }
            NodeMetadata::Endpoint | NodeMetadata::Email | NodeMetadata::File => {}
        }

        let connections_markdown = self
            .get_connections()
            .iter()
            .filter(|conn| !matches!(conn.r#type, Type::Endpoint))
            .map(Self::to_markdown)
            .collect::<Vec<String>>()
            .join("\n\n");

        let endpoint_connections = self
            .get_connections()
            .iter()
            .filter(|conn| matches!(conn.r#type, Type::Endpoint))
            // Later this may change, for now the endpoints only come from the Wayback Machine
            .map(|conn| {
                format!(
                    "- [`{0}`](https://web.archive.org/web/20250000000000*/{0})",
                    conn.value
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let mut sections = vec![format!("### {}", self.value)];
        if let Some(flags) = flags {
            sections.push(flags);
        }
        if !metadata_markdown.is_empty() {
            sections.push(metadata_markdown.trim().to_string());
        }
        if !data_markdown.is_empty() {
            sections.push(data_markdown.trim().to_string());
        }
        if !endpoint_connections.is_empty() {
            sections.push(format!("#### Endpoints\n{endpoint_connections}"));
        }
        if !connections_markdown.is_empty() {
            sections.push(connections_markdown);
        }
        sections.join("\n\n")
    }
}
