use std::{io::Error as IoError, process::ExitStatus, result::Result as StdResult};

use thiserror::Error as ThisError;
use toml::de::Error as TomlError;
use windows::core::Error as WindowsError;

use crate::core::show_error_dialog;

pub type Result<T, E = HamlibPttError> = StdResult<T, E>;

#[derive(Debug, ThisError)]
pub enum HamlibPttError {
    #[error("invariant broken")]
    InvalidState,

    #[error("error in Windows API: {0}")]
    Windows(#[from] WindowsError),

    #[error("failed to get DLL directory")]
    DllPath,

    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[error("config error: {0}")]
    ConfigSyntax(#[from] TomlError),

    #[error("invalid config data")]
    ConfigDataInvalid,

    #[error("rigctl failed with status {0}: {1}")]
    RigCtl(ExitStatus, String),
}

impl HamlibPttError {
    pub fn show_dialog(&self) {
        show_error_dialog(&self.to_string());
    }
}
