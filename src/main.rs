use std::{
    env::consts::{ARCH, OS},
    process,
};

use clap::Parser;
use logger::LOGGER;

mod args;
mod config;
mod database;
mod debug;
mod event_bus;
mod flags;
mod helpers;
mod logger;
mod modules;
mod session;
mod state;

fn main() {
    LOGGER.lock().unwrap().println(format!(
        "Project Absence v{} $[fg:gray](built for {OS} on {ARCH})$[effect:reset]",
        env!("CARGO_PKG_VERSION")
    ));

    let args = args::Args::parse();
    if args.version {
        process::exit(0);
    }

    let config = if args.modules.is_some() {
        match config::Config::from_default_with_overrides(&args.overrides) {
            Ok(config) => config,
            Err(e) => {
                logger::error("setup", e);
                process::exit(1);
            }
        }
    } else {
        config::create_file_if_not_existing();
        match args.parse_config() {
            Ok(config) => config,
            Err(e) => {
                logger::error("setup", e);
                process::exit(1);
            }
        }
    };

    let session = session::Session::new(args, config);
    session.register_config_modules();

    if let Err(err) = session.run() {
        logger::error("session", err.to_string());
    }
}
