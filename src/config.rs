use std::fs;
use std::path::PathBuf;

use anyhow::{Context as _, Result};
use dirs::config_dir;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct ViZshHistConfig {
    pub editor: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub vizshhist: ViZshHistConfig,
}

fn find_config_file() -> Option<PathBuf> {
    config_dir()
        .map(|config| config.join("vizshhist/config.toml"))
        .filter(|file| file.exists())
}

pub fn load_config() -> Result<Config> {
    let Some(config_file) = find_config_file() else {
        return Ok(Config::default());
    };
    toml::from_str(&fs::read_to_string(config_file)?).context("Configuration file error")
}
