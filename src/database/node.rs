use std::{collections::HashMap, fmt};

use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::flags;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Domain,
    Email,
}

impl fmt::Display for Type {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Domain => {
                write!(formatter, "domain")
            }
            Type::Email => {
                write!(formatter, "email")
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

#[derive(Clone, Debug, Serialize)]
pub struct Node {
    r#type: Type,
    value: String,
    connections: Vec<Node>,
    data: HashMap<String, Value>,
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
        Node {
            r#type,
            value,
            connections: Vec::new(),
            data: HashMap::new(),
        }
    }

    pub fn connect(&mut self, node: Node) {
        self.connections.push(node);
    }

    #[allow(unused)]
    pub fn add(&mut self, r#type: Type, value: String) {
        self.connect(Node::new(r#type, value));
    }

    pub fn add_data(&mut self, key: String, value: Value) {
        self.data.insert(key, value);
    }

    pub fn get_data(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn edit_data(&mut self, key: String, new_value: Value) {
        self.data.entry(key).and_modify(|e| *e = new_value);
    }

    pub fn add_flag(&mut self, flag: usize) {
        if !self.data.contains_key("flags") {
            self.data.insert(String::from("flags"), flags::ZERO.into());
        }
        let current_flags = self.get_data("flags").unwrap().as_u64().unwrap_or_default() as usize;
        self.edit_data(
            String::from("flags"),
            Value::Number((current_flags | flag).into()),
        );
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

    pub fn get_connections(&self) -> Vec<Node> {
        self.connections.clone()
    }

    pub fn find(&mut self, node: &Node) -> Option<&mut Node> {
        if self.equals(node) {
            return Some(self);
        }

        for connection in &mut self.connections {
            if connection.equals(node) {
                return Some(connection);
            } else if let Some(found) = connection.find(node) {
                return Some(found);
            }
        }

        None
    }

    pub fn equals(&self, other: &Node) -> bool {
        // Check type and value
        if self.r#type != other.r#type || self.value != other.value {
            return false;
        }

        // Check if all the connections are the same
        for (self_connection, other_connection) in self.connections.iter().zip(&other.connections) {
            if !self_connection.equals(other_connection) {
                return false;
            }
        }

        true
    }

    pub fn to_markdown(&self) -> String {
        let flags = if let Some(flags) = self.get_data("flags") {
            let mut result = String::from("#### Flags\n");
            result += format!(
                "\n- `IS_RECENT` => {}\n- `HAS_EXPIRED` => {}\n- `POSSIBLE_TAKEOVER` => {}",
                flags::contains_to_markdown(
                    flags.as_u64().unwrap() as usize,
                    flags::domain::IS_RECENT
                ),
                flags::contains_to_markdown(
                    flags.as_u64().unwrap() as usize,
                    flags::domain::HAS_EXPIRED
                ),
                if flags::contains(
                    flags.as_u64().unwrap() as usize,
                    flags::domain::POSSIBLE_TAKEOVER
                ) {
                    format!(
                        "✅ (Platform: `{}`)",
                        self.get_data("possible_takeover")
                            .unwrap_or(&Value::String("Not set".to_string()))
                            .as_str()
                            .unwrap_or_default()
                    )
                } else {
                    "❌".to_string()
                }
            )
            .as_str();
            Some(result)
        } else {
            None
        };

        let connections_markdown = self
            .get_connections()
            .iter()
            .map(|conn| conn.to_markdown())
            .collect::<Vec<String>>()
            .join("\n\n");

        let mut sections = vec![format!("### {}", self.value)];
        if let Some(flags) = flags {
            sections.push(flags);
        }
        if !connections_markdown.is_empty() {
            sections.push(connections_markdown);
        }
        sections.join("\n\n")
    }
}
