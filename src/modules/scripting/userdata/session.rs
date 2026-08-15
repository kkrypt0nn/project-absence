use mlua::{UserData, UserDataMethods};
use reqwest::header::USER_AGENT;
use std::sync::{Arc, Mutex};

use crate::event_bus::Event;
use crate::session::Session;
use crate::{database, helpers};

use super::database::LuaDatabase;

pub struct LuaSession {
    session: Arc<Session>,
    database: Arc<Mutex<database::Database>>,
}

impl LuaSession {
    pub fn new(session: Arc<Session>) -> Self {
        Self {
            database: Arc::clone(&session.get_database_arc()),
            session,
        }
    }
}

impl UserData for LuaSession {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("database", |_, this, ()| {
            Ok(LuaDatabase::new(Arc::clone(&this.database)))
        });

        methods.add_method("discover_domain", |_, this, domain: String| {
            this.session.publish(Event::DiscoveredDomain(domain));
            Ok(())
        });

        methods.add_method(
            "http_get",
            |_, this, (url, user_agent): (String, Option<String>)| {
                let ua = user_agent.unwrap_or(helpers::ua::get_random().to_string());
                let response = this
                    .session
                    .get_http_client()
                    .get(url)
                    .header(USER_AGENT, ua)
                    .send()
                    .map_err(mlua::Error::external)?;

                let body = response
                    .error_for_status()
                    .map_err(mlua::Error::external)?
                    .text()
                    .map_err(mlua::Error::external)?;
                Ok(body)
            },
        );
    }
}
