use std::process::Command;

use crate::core::{
    config::ConfigRig,
    error::{HamlibPttError, Result},
};

pub fn call_rigctl(rigctl_path: &str, rig: &ConfigRig, commands: Option<&[String]>) -> Result<()> {
    let Some(commands) = commands else {
        return Ok(());
    };
    if commands.is_empty() {
        return Ok(());
    }

    let mut rigctl = Command::new(rigctl_path)
        .arg("-m")
        .arg(rig.model_id.to_string())
        .arg("-r")
        .arg(&rig.device)
        .args(commands)
        .spawn()?;

    let status = rigctl.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(HamlibPttError::RigCtl(status))
    }
}
