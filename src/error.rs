use std::result::Result as StdResult;

use thiserror::Error as ThisError;
use windows::core::Error as WindowsError;

pub type Result<T, E = HamlibPttError> = StdResult<T, E>;

#[derive(Debug, ThisError)]
pub enum HamlibPttError {
    #[error("error in Windows API: {0}")]
    Windows(#[from] WindowsError),

    #[error("failed to get DLL directory")]
    DllPath,
}
