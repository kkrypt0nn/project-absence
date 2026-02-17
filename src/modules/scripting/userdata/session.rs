use mlua::{UserData, UserDataMethods};
use std::sync::{Arc, Mutex};

use crate::database;
use crate::session::Session;

use super::database::LuaDatabase;

pub struct LuaSession {
    database: Arc<Mutex<database::Database>>,
}

impl LuaSession {
    pub fn new(session: &Session) -> Self {
        Self {
            database: Arc::clone(&session.get_database_arc()),
        }
    }
}

impl UserData for LuaSession {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("database", |_, this, ()| {
            Ok(LuaDatabase::new(Arc::clone(&this.database)))
        });
    }
}
