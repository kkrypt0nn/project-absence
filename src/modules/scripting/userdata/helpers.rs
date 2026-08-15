use std::net::IpAddr;

use mlua::{UserData, UserDataMethods};

use crate::helpers;

pub struct LuaHelpers {}

impl LuaHelpers {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserData for LuaHelpers {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_ip_addr", |_, _, domain: String| {
            Ok(helpers::network::get_ip_addr(&domain).map(|ip| ip.to_string()))
        });

        methods.add_method("geolocate_ip", |lua, _, ip: String| {
            let ip = ip.parse::<IpAddr>().map_err(mlua::Error::external)?;
            let Some(geo_info) = helpers::network::geolocate_ip(ip) else {
                return Ok(mlua::Value::Nil);
            };

            let table = lua.create_table()?;
            table.set("city", geo_info.city)?;
            table.set("country", geo_info.country)?;

            Ok(mlua::Value::Table(table))
        });

        methods.add_method("random_ua", |_, _, ()| Ok(helpers::ua::get_random()));
    }
}
