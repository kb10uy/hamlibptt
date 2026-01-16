use std::path::Path;

use crate::{
    core::{
        config::{ConfigControlMode, load_config},
        error::{HamlibPttError, Result},
        show_info_dialog,
    },
    extfsk::{
        close_commander, initialize_backend, parameter::ExtfskParameter, send_hamlib_command,
    },
    hamlib::{
        HamlibCommander,
        rigctl::{RetainedRigctlCommander, RigctlCommander},
        rigctld::{RetainedRigctldCommander, RigctldCommander},
    },
};

pub fn open(dll_directory: &Path, _parameter: ExtfskParameter) -> Result<()> {
    let config = load_config(dll_directory)?;

    let (commander, status): (Box<dyn HamlibCommander>, _) = match config.mode {
        ConfigControlMode::Rigctl => {
            let Some(rigctl) = &config.rigctl else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            (
                Box::new(RigctlCommander::new(rigctl)),
                format!(
                    "rigctl at {} (id={}, device={})",
                    rigctl.rigctl_path, rigctl.model_id, rigctl.device
                ),
            )
        }
        ConfigControlMode::RigctlRetained => {
            let Some(rigctl) = &config.rigctl else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            (
                Box::new(RetainedRigctlCommander::new(rigctl)?),
                format!(
                    "retained connection rigctl at {} (id={}, device={})",
                    rigctl.rigctl_path, rigctl.model_id, rigctl.device
                ),
            )
        }
        ConfigControlMode::Rigctld => {
            let Some(rigctld) = &config.rigctld else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            (
                Box::new(RigctldCommander::new(rigctld)),
                format!("rigctld on {}", rigctld.address),
            )
        }
        ConfigControlMode::RigctldRetained => {
            let Some(rigctld) = &config.rigctld else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            (
                Box::new(RetainedRigctldCommander::new(rigctld)?),
                format!("retained connection rigctld on {}", rigctld.address),
            )
        }
    };

    initialize_backend(commander, config.commands.clone());
    send_hamlib_command(|cmds| cmds.open.as_deref().unwrap_or_default())?;
    show_info_dialog(&format!("Initialized successfully!\nOperating {status}"));

    Ok(())
}

pub fn close() -> Result<()> {
    send_hamlib_command(|cmds| cmds.close.as_deref().unwrap_or_default())?;
    close_commander()?;
    Ok(())
}
