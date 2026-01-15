mod core;
mod extfsk;

use std::ffi::{c_int, c_long, c_uchar, c_ulong};

use windows::Win32::{
    Foundation::HMODULE,
    System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

use crate::{
    core::dll::{DLL_DIRECTORY, on_dll_attach, on_dll_detach},
    extfsk::{
        ExtfskParameter, close,
        fsk::{is_busy, put_char},
        open,
        ptt::set_ptt,
    },
};

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(module: HMODULE, call_reason: c_ulong, _: *mut ()) -> c_int {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            on_dll_attach(module).ok();
        }
        DLL_PROCESS_DETACH => {
            extfskClose();
            on_dll_detach().ok();
        }
        _ => (),
    }
    1
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskOpen(parameter: c_long) -> c_int {
    let Some(dll_directory) = DLL_DIRECTORY.get() else {
        return 0;
    };

    match open(
        dll_directory,
        ExtfskParameter::parse(parameter.cast_unsigned()),
    ) {
        Ok(()) => 1,
        Err(e) => {
            e.show_dialog();
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskClose() {
    match close() {
        Ok(()) => (),
        Err(e) => {
            e.show_dialog();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskIsTxBusy() -> c_long {
    if is_busy().unwrap_or(false) { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskPutChar(c: c_uchar) {
    put_char(c).ok();
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskSetPTT(tx: c_long) {
    match set_ptt(tx != 0) {
        Ok(()) => (),
        Err(e) => {
            e.show_dialog();
        }
    }
}
