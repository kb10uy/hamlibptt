use crate::core::{config::Config, error::Result};

pub fn is_busy(config: &Config) -> Result<bool> {
    Ok(false)
}

pub fn put_char(config: &Config, c: u8) -> Result<()> {
    Ok(())
}
