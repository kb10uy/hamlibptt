use std::{
    io::{BufRead, BufReader, Write},
    net::{Shutdown, TcpStream},
};

use crate::{
    core::{config::ConfigRigctld, error::Result},
    hamlib::{HamlibCommander, HamlibError},
};

#[derive(Debug)]
pub struct RigctldCommander {
    config: ConfigRigctld,
}

impl RigctldCommander {
    pub fn new(config: &ConfigRigctld) -> RigctldCommander {
        RigctldCommander {
            config: config.clone(),
        }
    }
}

impl HamlibCommander for RigctldCommander {
    fn send(&mut self, commands: &[String]) -> Result<(), HamlibError> {
        let mut command_str = commands.join(" ");
        command_str.push('\n');

        let mut stream = TcpStream::connect(self.config.address)?;
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

        let code = response_line
            .trim()
            .split_once(" ")
            .map(|(_, c)| c)
            .unwrap_or("(cannot decode)");
        if code != "0" {
            return Err(HamlibError::Control(format!(
                "rigctld returned error {code}"
            )));
        }

        Ok(())
    }

    fn close(&mut self) -> Result<(), HamlibError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct RetainedRigctldCommander {
    stream: TcpStream,
}

impl RetainedRigctldCommander {
    pub fn new(config: &ConfigRigctld) -> Result<RetainedRigctldCommander> {
        let stream = TcpStream::connect(config.address)?;
        Ok(RetainedRigctldCommander { stream })
    }
}

impl HamlibCommander for RetainedRigctldCommander {
    fn send(&mut self, commands: &[String]) -> Result<(), HamlibError> {
        let mut command_str = commands.join(" ");
        command_str.push('\n');

        self.stream.write_all(command_str.as_bytes())?;
        self.stream.flush()?;

        let mut response_reader = BufReader::new(self.stream.try_clone()?);
        let mut response_line = String::new();
        loop {
            response_line.clear();
            response_reader.read_line(&mut response_line)?;
            if response_line.starts_with("RPRT") {
                break;
            }
        }

        let code = response_line
            .trim()
            .split_once(" ")
            .map(|(_, c)| c)
            .unwrap_or("(cannot decode)");
        if code != "0" {
            return Err(HamlibError::Control(format!(
                "rigctld returned error {code}"
            )));
        }

        Ok(())
    }

    fn close(&mut self) -> Result<(), HamlibError> {
        self.stream.shutdown(Shutdown::Both).ok();
        Ok(())
    }
}

impl Drop for RetainedRigctldCommander {
    fn drop(&mut self) {
        self.close().ok();
    }
}
