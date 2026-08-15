use crate::modules::scripting::userdata::{helpers::LuaHelpers, logger::LuaLogger};
use mlua::{Lua, Result as LuaResult};

pub fn register(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    globals.set("helpers", LuaHelpers::new())?;
    globals.set("logger", LuaLogger::new())?;

    Ok(())
}
