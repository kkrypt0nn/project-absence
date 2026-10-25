use node::{Node, Type};

use crate::database::node::{DomainData, NodeMetadata};

pub mod node;
#[cfg(test)]
mod tests;

pub struct Database {
    root: Node,
}

impl Database {
    pub fn new(mut root: Node) -> Self {
        if root.r#type == Type::Domain {
            root.metadata = NodeMetadata::Domain(DomainData { is_root: true });
        }
        Self { root }
    }

    #[allow(dead_code)]
    pub const fn get_root(&mut self) -> &mut Node {
        &mut self.root
    }

    #[allow(dead_code)]
    pub fn get_as_json(&self) -> String {
        serde_json::to_string(&self.root).unwrap()
    }

    pub fn get_as_pretty_json(&self) -> String {
        serde_json::to_string_pretty(&self.root).unwrap()
    }

    pub fn search(&mut self, r#type: Type, value: String) -> Option<&mut Node> {
        self.root.find(&r#type, &value)
    }
}
