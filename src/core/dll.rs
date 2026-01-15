use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf, sync::OnceLock};

use windows::Win32::{Foundation::HMODULE, System::LibraryLoader::GetModuleFileNameW};

use crate::core::error::{HamlibPttError, Result};

pub static DLL_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();

pub fn on_dll_attach(module: HMODULE) -> Result<()> {
    fetch_dll_directory(module)?;
    Ok(())
}

pub fn on_dll_detach() -> Result<()> {
    Ok(())
}

fn fetch_dll_directory(module: HMODULE) -> Result<()> {
    let dll_path = unsafe {
        let mut buffer = vec![0; 512];
        let length = GetModuleFileNameW(Some(module), &mut buffer) as usize;
        PathBuf::from(OsString::from_wide(&buffer[..length - 1]))
    };

    let Some(dll_directory) = dll_path.parent() else {
        return Err(HamlibPttError::DllPath);
    };
    DLL_DIRECTORY.set(dll_directory.to_path_buf()).ok();

    Ok(())
}
