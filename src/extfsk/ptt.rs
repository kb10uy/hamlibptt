use crate::{core::error::Result, extfsk::run_command};

pub fn set_ptt(tx: bool, _mmsstv_scan: bool) -> Result<()> {
    if tx {
        run_command(|cmds| cmds.tx.as_deref().unwrap_or_default())
    } else {
        run_command(|cmds| cmds.rx.as_deref().unwrap_or_default())
    }
}
