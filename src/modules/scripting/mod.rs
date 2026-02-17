use std::fs;

use mlua::Function;

use crate::event_bus::Event;
use crate::modules::Module;
use crate::modules::scripting::userdata::event::LuaEvent;
use crate::modules::scripting::userdata::session::LuaSession;
use crate::session::Session;

mod globals;
mod userdata;

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
        globals::register(&self.lua).map_err(|e| e.to_string())
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

    fn subscribers(&self) -> Vec<String> {
        self.module
            .get::<Function>("subscribers")
            .unwrap()
            .call::<Vec<String>>("")
            .unwrap_or_default()
    }

    fn execute(&self, session: &Session, event: &Event) -> Result<(), String> {
        if let Ok(execute_fn) = self.module.get::<mlua::Function>("execute")
            && let Err(e) = execute_fn.call::<bool>((
                self.name(),
                LuaSession::new(session),
                LuaEvent::new(event),
            ))
        {
            return Err(e.to_string());
        }
        Ok(())
    }
}
