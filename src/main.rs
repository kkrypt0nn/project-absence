use std::{
    env::consts::{ARCH, OS},
    process,
    sync::mpsc,
};

use clap::Parser;
use logger::LOGGER;

mod args;
mod config;
mod database;
mod debug;
mod events;
mod flags;
mod helpers;
mod logger;
mod modules;
mod session;
mod state;

fn main() {
    config::create_file_if_not_existing();
    LOGGER.lock().unwrap().println(format!(
        "Project Absence v{} $[fg:gray](built for {} on {})$[effect:reset]",
        env!("CARGO_PKG_VERSION"),
        OS,
        ARCH
    ));

    let args = args::Args::parse();
    if args.version {
        process::exit(0);
    }
    let config = match args.parse_config() {
        Ok(config) => config,
        Err(e) => {
            logger::error("setup", e);
            process::exit(1);
        }
    };

    let (tx, rx) = mpsc::sync_channel::<events::Type>(100);
    let session = session::Session::new(args, config, tx, rx);
    session.register_config_modules();

    if let Err(err) = session.run() {
        logger::error("session", err.to_string());
    }
}
