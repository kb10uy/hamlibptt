mod config;
mod dll;
mod error;

use std::ffi::{c_int, c_long, c_uchar, c_ulong};

use windows::{
    Win32::{
        Foundation::HMODULE,
        System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        UI::WindowsAndMessaging::{MB_ICONINFORMATION, MB_OK, MessageBoxA},
    },
    core::{PCSTR, s},
};

use crate::{
    config::{CONFIG, load_config},
    dll::DLL_DIRECTORY,
};

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(module: HMODULE, call_reason: c_ulong, _: *mut ()) -> c_int {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            dll::on_dll_attach(module).ok();
        }
        DLL_PROCESS_DETACH => {
            extfskClose();
            dll::on_dll_detach().ok();
        }
        _ => (),
    }
    1
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskOpen(_parameter: c_long) {
    let Some(dll_directory) = DLL_DIRECTORY.get() else {
        return;
    };
    match load_config(dll_directory) {
        Ok(()) => (),
        Err(e) => {
            unsafe {
                MessageBoxA(
                    None,
                    PCSTR(e.to_string().as_ptr()),
                    s!("HamlibPTT"),
                    MB_ICONINFORMATION | MB_OK,
                );
            }
            return;
        }
    }

    if let Some(config) = CONFIG.get() {
        unsafe {
            MessageBoxA(
                None,
                PCSTR(format!("{config:?}").as_ptr()),
                s!("HamlibPTT"),
                MB_ICONINFORMATION | MB_OK,
            );
        }
    };
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskClose() {
    println!("Close");
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskIsTxBusy() -> c_long {
    println!("IsTxBusy?");
    0
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskPutChar(c: c_uchar) {
    println!("PutChar: {c}");
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskSetPTT(tx: c_long) {
    println!("SetPTT: {tx}");
}
