use mlua::{UserData, UserDataMethods};

use crate::logger;

pub struct LuaLogger {}

impl LuaLogger {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserData for LuaLogger {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("println", |_, _, message: String| {
            logger::println(String::from("lua:script"), message);
            Ok(())
        });
    }
}
