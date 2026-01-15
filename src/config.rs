use serde::Deserialize;

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
