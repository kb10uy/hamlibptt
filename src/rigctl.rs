use std::{
    os::windows::process::CommandExt,
    process::{Command, Stdio},
};

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

    let rigctl = Command::new(rigctl_path)
        .creation_flags(0x08000000)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .arg("-m")
        .arg(rig.model_id.to_string())
        .arg("-r")
        .arg(&rig.device)
        .args(commands)
        .spawn()?;

    let output = rigctl.wait_with_output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(HamlibPttError::RigCtl(output.status, stderr.to_string()))
    }
}
