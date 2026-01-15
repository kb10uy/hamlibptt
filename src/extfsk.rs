pub mod fsk;
pub mod ptt;

use std::path::Path;

use crate::core::{
    config::{CONFIG, load_config},
    error::Result,
    show_info_dialog,
};

pub fn open(dll_directory: &Path) -> Result<()> {
    load_config(dll_directory)?;

    if let Some(config) = CONFIG.get() {
        show_info_dialog(&format!("{config:#?}"));
    };

    Ok(())
}

pub fn close() -> Result<()> {
    Ok(())
}
