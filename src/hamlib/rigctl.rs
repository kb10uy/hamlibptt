use std::{
    io::Write,
    os::windows::process::CommandExt,
    process::{Child, ChildStdin, Command, Stdio},
};

use crate::{
    core::{config::ConfigRigctl, error::Result},
    hamlib::{HamlibCommander, HamlibError},
};

/// Commander which uses instant rigctl processes.
#[derive(Debug)]
pub struct RigctlCommander {
    config: ConfigRigctl,
}

impl RigctlCommander {
    pub fn new(config: &ConfigRigctl) -> RigctlCommander {
        RigctlCommander {
            config: config.clone(),
        }
    }
}

impl HamlibCommander for RigctlCommander {
    fn send(&mut self, commands: &[String]) -> Result<(), HamlibError> {
        let rigctl = Command::new(&self.config.rigctl_path)
            .creation_flags(0x08000000)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .arg("-m")
            .arg(self.config.model_id.to_string())
            .arg("-r")
            .arg(&self.config.device)
            .args(commands)
            .spawn()?;

        let output = rigctl.wait_with_output()?;
        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(HamlibError::Control(stderr.to_string()))
        }
    }

    fn close(&mut self) -> Result<(), HamlibError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct RetainedRigctlCommander {
    process: Child,
    stdin: ChildStdin,
}

impl RetainedRigctlCommander {
    pub fn new(config: &ConfigRigctl) -> Result<RetainedRigctlCommander> {
        let mut process = Command::new(&config.rigctl_path)
            .creation_flags(0x08000000)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .arg("-m")
            .arg(config.model_id.to_string())
            .arg("-r")
            .arg(&config.device)
            .spawn()?;
        let stdin = process.stdin.take().expect("should have stdin");

        Ok(RetainedRigctlCommander { process, stdin })
    }
}

impl HamlibCommander for RetainedRigctlCommander {
    fn send(&mut self, commands: &[String]) -> Result<(), HamlibError> {
        let mut command_str = commands.join(" ");
        command_str.push('\n');

        self.stdin.write_all(command_str.as_bytes())?;
        Ok(())
    }

    fn close(&mut self) -> Result<(), HamlibError> {
        self.stdin.write_all(b"q\n")?;
        self.process.wait()?;
        Ok(())
    }
}

impl Drop for RetainedRigctlCommander {
    fn drop(&mut self) {
        self.close().ok();
    }
}
