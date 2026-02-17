use crate::modules::scripting::userdata::logger::LuaLogger;
use mlua::{Lua, Result as LuaResult};

pub fn register(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    globals.set("logger", LuaLogger::new())?;

    Ok(())
}
