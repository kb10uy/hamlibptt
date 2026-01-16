use crate::{core::error::Result, extfsk::operate_fsk};

pub fn is_busy() -> Result<bool> {
    Ok(operate_fsk(|f| f.is_busy())?.unwrap_or(false))
}

pub fn put_char(c: u8) -> Result<()> {
    operate_fsk(|f| f.send(c))?;
    Ok(())
}
