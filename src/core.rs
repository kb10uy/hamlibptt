pub mod config;
pub mod dll;
pub mod error;

use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use windows::{
    Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_ICONINFORMATION, MB_OK, MessageBoxW},
    core::{PCWSTR, w},
};

pub fn show_info_dialog(message: &str) {
    let message: Vec<_> = OsString::from(message).encode_wide().collect();
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(message.as_ptr()),
            w!("HamlibPTT"),
            MB_ICONINFORMATION | MB_OK,
        );
    }
}

pub fn show_error_dialog(message: &str) {
    let message: Vec<_> = OsString::from(message).encode_wide().collect();
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(message.as_ptr()),
            w!("HamlibPTT"),
            MB_ICONERROR | MB_OK,
        );
    }
}
