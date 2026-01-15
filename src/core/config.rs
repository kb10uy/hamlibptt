use std::{fs::read_to_string, path::Path, sync::OnceLock};

use serde::Deserialize;

use crate::core::error::Result;

pub const CONFIG_FILENAME: &str = "hamlibptt.toml";
pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub rigctl_path: String,
    pub rig: ConfigRig,
    pub commands: ConfigCommands,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigRig {
    pub model_id: usize,
    pub device: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigCommands {
    pub open: Option<Vec<String>>,
    pub close: Option<Vec<String>>,
    pub tx: Option<Vec<String>>,
    pub rx: Option<Vec<String>>,
}

pub fn load_config(directory: &Path) -> Result<()> {
    let config_path = directory.join(CONFIG_FILENAME);
    let config_toml = read_to_string(config_path)?;
    let config = toml::from_str(&config_toml)?;

    CONFIG.set(config).ok();
    Ok(())
}
