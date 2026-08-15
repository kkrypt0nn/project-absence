use std::sync::{Arc, Mutex};

use mlua::{UserData, UserDataMethods};

use crate::database::{
    self,
    node::{Node, Type},
};

pub struct LuaDatabase {
    database: Arc<Mutex<database::Database>>,
}

impl LuaDatabase {
    pub fn new(database: Arc<Mutex<database::Database>>) -> Self {
        Self { database }
    }
}

impl UserData for LuaDatabase {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut(
            "add_data",
            |_, this, (type_str, node_name, key, value): (String, String, String, String)| {
                let node_type = match type_str.as_str() {
                    "domain" => Type::Domain,
                    "endpoint" => Type::Endpoint,
                    "email" => Type::Email,
                    _ => return Ok(()),
                };

                let mut db = this.database.lock().unwrap();

                let node = if let Some(node_ref) = db.search(node_type, node_name) {
                    node_ref
                } else {
                    return Ok(());
                };

                node.add_data(key, serde_json::Value::String(value));
                Ok(())
            },
        );

        methods.add_method_mut(
            "connect_new_node",
            |_,
             this,
             (parent_node_type, parent_node_name, type_str, node_name): (
                String,
                String,
                String,
                String,
            )| {
                let parent_node_type = match parent_node_type.as_str() {
                    "domain" => Type::Domain,
                    "endpoint" => Type::Endpoint,
                    "email" => Type::Email,
                    _ => return Ok(()),
                };
                let new_node_type = match type_str.as_str() {
                    "domain" => Type::Domain,
                    "endpoint" => Type::Endpoint,
                    "email" => Type::Email,
                    _ => return Ok(()),
                };

                let mut db = this.database.lock().unwrap();

                if let Some(parent) = db.search(parent_node_type, parent_node_name) {
                    parent.connect(Node::new(new_node_type, node_name));
                }

                Ok(())
            },
        );
    }
}
