mod core;
mod extfsk;

use std::ffi::{c_int, c_long, c_uchar, c_ulong};

use windows::Win32::{
    Foundation::HMODULE,
    System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

use crate::{
    core::dll::{DLL_DIRECTORY, on_dll_attach, on_dll_detach},
    extfsk::{close, open},
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
pub extern "system" fn extfskOpen(_parameter: c_long) {
    let Some(dll_directory) = DLL_DIRECTORY.get() else {
        return;
    };

    match open(dll_directory) {
        Ok(()) => (),
        Err(e) => {
            e.show_dialog();
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
