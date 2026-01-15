pub mod rigctl;
pub mod rigctld;

use std::io::Error as IoError;

use thiserror::Error as ThisError;

pub trait HamlibCommander: Send + Sync {
    fn send(&mut self, commands: &[String]) -> Result<(), HamlibError>;
    fn close(&mut self) -> Result<(), HamlibError>;
}

#[derive(Debug, ThisError)]
pub enum HamlibError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[error("control error: {0}")]
    Control(String),
}
