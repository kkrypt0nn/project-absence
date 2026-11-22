use std::time::Duration;

use semver::Version;

use crate::logger;

pub fn check_version() {
    let current_raw = env!("CARGO_PKG_VERSION");

    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
    {
        Ok(client) => client,
        Err(_) => return,
    };
    let latest_raw = match client
        .get("https://absence.krypton.ninja/latest-version")
        .send()
    {
        Ok(response) if response.status().is_success() => match response.text() {
            Ok(version) => version.trim().to_string(),
            Err(_) => return,
        },
        _ => return,
    };

    let current_version = match Version::parse(current_raw) {
        Ok(version) => version,
        Err(_) => return,
    };

    let latest_version = match Version::parse(&latest_raw) {
        Ok(version) => version,
        Err(_) => return,
    };

    if latest_version > current_version {
        logger::LOGGER.lock().unwrap().print(format!(
            " $[fg:yellow](update available: v{current_version} → v{latest_version})$[effect:reset]"
        ));
    }
}
