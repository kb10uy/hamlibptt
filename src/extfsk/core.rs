use std::path::Path;

use crate::{
    core::{
        config::{CONFIG, Config, load_config},
        error::Result,
        show_info_dialog,
    },
    extfsk::ExtfskParameter,
};

pub fn open(dll_directory: &Path, _parameter: ExtfskParameter) -> Result<()> {
    load_config(dll_directory)?;

    if let Some(config) = CONFIG.get() {
        show_info_dialog(&format!("{config:#?}"));
    };

    Ok(())
}

pub fn close(config: &Config) -> Result<()> {
    Ok(())
}
