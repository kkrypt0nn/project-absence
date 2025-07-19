use std::net::{IpAddr, ToSocketAddrs};

use serde::{Deserialize, Serialize};

pub fn get_ip_addr(domain: &str) -> Option<IpAddr> {
    // 1337 is just a dummy port because apparently it absolutely needs one
    match format!("{}:1337", domain).to_socket_addrs() {
        Ok(mut socket_addr) => socket_addr.next().map(|addr| addr.ip()),
        Err(_) => None,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeoInfo {
    city: Option<String>,
    country: Option<String>,
}

impl From<GeoInfo> for serde_json::Value {
    fn from(value: GeoInfo) -> Self {
        serde_json::Value::Object(serde_json::Map::from_iter([
            (String::from("city"), value.city.into()),
            (String::from("country"), value.country.into()),
        ]))
    }
}

pub fn geolocate_ip(ip: IpAddr) -> Option<GeoInfo> {
    // TODO: Maybe use a local DB? For now that's fine though :)
    reqwest::blocking::get(format!("http://ip-api.com/json/{}?fields=city,country", ip))
        .ok()?
        .json::<GeoInfo>()
        .ok()
}
