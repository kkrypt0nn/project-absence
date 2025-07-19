use std::{env, fs, path::PathBuf};

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, Clone, Default, Parser)]
#[clap(
    author = "Krypton (https://krypton.ninja)",
    about,
    arg_required_else_help(true)
)]
pub struct Args {
    /// Domain to scan for
    #[arg(
        short = 'd',
        long,
        default_value = "",
        required_unless_present = "version"
    )]
    pub domain: String,

    /// Path to configuration file
    #[arg(short = 'c', long, default_value = "~/.absence/config.toml")]
    pub config: String,

    /// The path of where the output will be saved to
    #[arg(short = 'o', long, default_value = "~/.absence")]
    pub output: String,

    /// Whether to copy the resulting JSON database to the clipboard
    #[cfg(feature = "clipboard")]
    #[arg(short = 'C', long, default_value_t = false)]
    pub clipboard: bool,

    /// The file path of the Lua script to load
    #[arg(short = 's', long)]
    pub script: Option<String>,

    /// Display the verison of the tool
    #[arg(short = 'V', long, default_value_t = false)]
    pub version: bool,

    /// Whether to print the database at the end of execution in a tree format and some other debugging data
    #[arg(short = 'D', long, default_value_t = false)]
    pub debug: bool,

    /// Whether to print some verbose data
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,
}

impl Args {
    pub fn parse_config(&self) -> Result<Config, String> {
        let home_dir = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .unwrap_or_else(|_| String::from(""));
        let config_path = &self.config;
        let expanded_config_path = if config_path.starts_with("~") {
            let mut expanded_path = config_path.clone();
            expanded_path.replace_range(0..1, &home_dir);
            PathBuf::from(expanded_path)
        } else {
            PathBuf::from(config_path)
        };
        let toml_content = fs::read_to_string(&expanded_config_path).map_err(|e| {
            format!(
                "Failed to read config file ({:?}): {}",
                expanded_config_path, e
            )
        })?;
        toml::from_str(&toml_content).map_err(|e| format!("Failed to parse TOML config: {}", e))
    }
}
