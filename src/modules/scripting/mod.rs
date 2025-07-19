use std::fs;

use mlua::Function;

use crate::modules::{Context, Module};
use crate::session::Session;
use crate::{events, logger};

pub struct Scripting {
    lua: mlua::Lua,
    module: mlua::Table,
}

impl Scripting {
    pub fn new(script_path: &str) -> Result<Self, String> {
        let lua = mlua::Lua::new();
        let script = fs::read_to_string(script_path).map_err(|e| e.to_string())?;
        let module: mlua::Table = lua.load(&script).eval().map_err(|e| e.to_string())?;
        let mluascript = Self { lua, module };
        mluascript.setup_globals().map_err(|e| e.to_string())?;
        Ok(mluascript)
    }

    fn setup_globals(&self) -> Result<(), String> {
        // TODO: Expose more functions and structs
        // Maybe worth having an 'impl IntoLua' for all the special structs, e.g. the database nodes
        let globals = self.lua.globals();
        globals
            .set(
                "println",
                self.lua
                    .create_function(move |_, message: String| {
                        logger::println(String::from("lua:script"), message);
                        Ok(())
                    })
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl Module for Scripting {
    fn name(&self) -> String {
        String::from("scripting")
    }

    fn description(&self) -> String {
        self.module
            .get::<Function>("description")
            .unwrap()
            .call::<String>("")
            .unwrap_or(String::from(
                "This module is responsible to execute a Lua script.",
            ))
    }

    fn subscribers(&self) -> Vec<events::Type> {
        if let Ok(subs) = self
            .module
            .get::<Function>("subscribers")
            .unwrap()
            .call::<Vec<String>>("")
        {
            subs.into_iter()
                .filter_map(|s| match s.as_str() {
                    "Ready" => Some(events::Type::Ready),
                    "DiscoveredDomain" => Some(events::Type::DiscoveredDomain(String::new())),
                    "OpenPort" => Some(events::Type::OpenPort(String::new(), 0)),
                    _ => None,
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn execute(&self, _: &Session, _: Context) -> Result<(), String> {
        if let Ok(execute_fn) = self.module.get::<mlua::Function>("execute") {
            // The session methods should be made globally availble. Likely as a table
            // The context args should be passed as a of string, convert everthing
            if let Err(e) = execute_fn.call::<bool>("") {
                return Err(e.to_string());
            }
        }
        Ok(())
    }
}
