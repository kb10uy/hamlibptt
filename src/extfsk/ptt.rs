use crate::{
    core::{config::Config, error::Result},
    rigctl::call_rigctl,
};

pub fn set_ptt(config: &Config, tx: bool) -> Result<()> {
    if tx {
        call_rigctl(
            &config.rigctl_path,
            &config.rig,
            config.commands.tx.as_deref(),
        )
    } else {
        call_rigctl(
            &config.rigctl_path,
            &config.rig,
            config.commands.rx.as_deref(),
        )
    }
}
