use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    os::windows::process::CommandExt,
    process::{Command, Stdio},
};

use crate::core::{
    config::{Config, ConfigControlMode, ConfigRigctl, ConfigRigctld},
    error::{HamlibPttError, Result},
};

pub fn call(config: &Config, commands: Option<&[String]>) -> Result<()> {
    let Some(commands) = commands else {
        return Ok(());
    };
    if commands.is_empty() {
        return Ok(());
    }

    match config.mode {
        ConfigControlMode::Rigctl => {
            let Some(rigctl) = &config.rigctl else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            call_rigctl(rigctl, commands)
        }
        ConfigControlMode::Rigctld => {
            let Some(rigctld) = &config.rigctld else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            call_rigctld(rigctld, commands)
        }
    }
}

fn call_rigctl(rigctl: &ConfigRigctl, commands: &[String]) -> Result<()> {
    let rigctl = Command::new(&rigctl.rigctl_path)
        .creation_flags(0x08000000)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .arg("-m")
        .arg(rigctl.model_id.to_string())
        .arg("-r")
        .arg(&rigctl.device)
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

fn call_rigctld(rigctld: &ConfigRigctld, commands: &[String]) -> Result<()> {
    let mut command_str = commands.join(" ");
    command_str.push('\n');

    let mut stream = TcpStream::connect(rigctld.address)?;
    stream.write_all(command_str.as_bytes())?;
    stream.flush()?;

    let mut response_reader = BufReader::new(stream);
    let mut response_line = String::new();
    loop {
        response_line.clear();
        response_reader.read_line(&mut response_line)?;
        if response_line.starts_with("RPRT") {
            break;
        }
    }

    Ok(())
}
