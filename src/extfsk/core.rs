use std::path::Path;

use crate::{
    core::{
        config::{CONFIG, Config, load_config},
        error::Result,
        show_info_dialog,
    },
    extfsk::ExtfskParameter,
    rigctl::call_rigctl,
};

pub fn open(dll_directory: &Path, _parameter: ExtfskParameter) -> Result<()> {
    load_config(dll_directory)?;

    if let Some(config) = CONFIG.get() {
        show_info_dialog(&format!("{config:#?}"));
        call_rigctl(
            &config.rigctl_path,
            &config.rig,
            config.commands.open.as_deref(),
        )?;
    };

    Ok(())
}

pub fn close(config: &Config) -> Result<()> {
    call_rigctl(
        &config.rigctl_path,
        &config.rig,
        config.commands.close.as_deref(),
    )?;
    Ok(())
}
