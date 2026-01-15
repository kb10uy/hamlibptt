use std::path::Path;

use crate::{
    core::{
        config::{CONFIG, Config, ConfigControlMode, load_config},
        error::{HamlibPttError, Result},
        show_info_dialog,
    },
    extfsk::ExtfskParameter,
    rigctl::call,
};

pub fn open(dll_directory: &Path, _parameter: ExtfskParameter) -> Result<()> {
    load_config(dll_directory)?;
    let Some(config) = CONFIG.get() else {
        return Err(HamlibPttError::InvalidState);
    };

    let message = match config.mode {
        ConfigControlMode::Rigctl => {
            let Some(rigctl) = &config.rigctl else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            format!(
                "rigctl at {} (id={}, device={})",
                rigctl.rigctl_path, rigctl.model_id, rigctl.device
            )
        }
        ConfigControlMode::Rigctld => {
            let Some(rigctld) = &config.rigctld else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            format!("rigctld on {}", rigctld.address)
        }
    };

    show_info_dialog(&format!("Initialized successfully!\nOperating {message}"));
    call(config, config.commands.open.as_deref())?;

    Ok(())
}

pub fn close(config: &Config) -> Result<()> {
    call(config, config.commands.close.as_deref())?;
    Ok(())
}
