use crate::{
    core::{config::Config, error::Result},
    rigctl::call,
};

pub fn set_ptt(config: &Config, tx: bool) -> Result<()> {
    if tx {
        call(config, config.commands.tx.as_deref())
    } else {
        call(config, config.commands.rx.as_deref())
    }
}
