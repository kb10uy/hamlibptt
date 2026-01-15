use std::{io::Error as IoError, process::ExitStatus, result::Result as StdResult};

use thiserror::Error as ThisError;
use toml::de::Error as TomlError;
use windows::core::Error as WindowsError;

use crate::core::show_error_dialog;

pub type Result<T, E = HamlibPttError> = StdResult<T, E>;

#[derive(Debug, ThisError)]
pub enum HamlibPttError {
    #[error("error in Windows API: {0}")]
    Windows(#[from] WindowsError),

    #[error("failed to get DLL directory")]
    DllPath,

    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[error("config error: {0}")]
    Config(#[from] TomlError),

    #[error("rigctl failed: {0}")]
    RigCtl(ExitStatus),
}

impl HamlibPttError {
    pub fn show_dialog(&self) {
        show_error_dialog(&self.to_string());
    }
}
