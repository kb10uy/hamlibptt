use crate::{core::error::Result, extfsk::send_hamlib_command};

pub fn set_ptt(tx: bool, _mmsstv_scan: bool) -> Result<()> {
    if tx {
        send_hamlib_command(|cmds| cmds.tx.as_deref().unwrap_or_default())
    } else {
        send_hamlib_command(|cmds| cmds.rx.as_deref().unwrap_or_default())
    }
}
