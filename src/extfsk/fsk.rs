use crate::core::{config::Config, error::Result};

pub fn is_busy(_config: &Config) -> Result<bool> {
    Ok(false)
}

pub fn put_char(_config: &Config, _c: u8) -> Result<()> {
    Ok(())
}
