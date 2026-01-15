mod core;
mod extfsk;

use std::ffi::{c_int, c_long, c_uchar, c_ulong};

use windows::Win32::{
    Foundation::HMODULE,
    System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

use crate::{
    core::{
        config::CONFIG,
        dll::{DLL_DIRECTORY, on_dll_attach, on_dll_detach},
    },
    extfsk::{
        ExtfskParameter,
        core::{close, open},
        fsk::{is_busy, put_char},
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
    let Some(config) = CONFIG.get() else {
        return;
    };

    match close(config) {
        Ok(()) => (),
        Err(e) => {
            e.show_dialog();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskIsTxBusy() -> c_long {
    let Some(config) = CONFIG.get() else {
        return 0;
    };

    if is_busy(config).unwrap_or(false) {
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskPutChar(c: c_uchar) {
    let Some(config) = CONFIG.get() else {
        return;
    };

    put_char(config, c).ok();
}

#[unsafe(no_mangle)]
pub extern "system" fn extfskSetPTT(tx: c_long) {
    let Some(config) = CONFIG.get() else {
        return;
    };

    match set_ptt(config, tx != 0) {
        Ok(()) => (),
        Err(e) => {
            e.show_dialog();
        }
    }
}
