use std::sync::{Arc, Mutex};

use mlua::{UserData, UserDataMethods};

use crate::database::{self, node::Type};

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
    }
}
