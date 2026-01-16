pub mod core;
pub mod fsk;
pub mod parameter;
pub mod ptt;

use std::sync::{Mutex, OnceLock};

use crate::{
    core::{config::ConfigCommands, error::Result},
    hamlib::HamlibCommander,
    spinfsk::SpinFsk,
};

static HAMLIB_COMMANDER: OnceLock<Mutex<Box<dyn HamlibCommander>>> = OnceLock::new();
static SPIN_FSK: OnceLock<Mutex<SpinFsk>> = OnceLock::new();
static COMMANDS: OnceLock<ConfigCommands> = OnceLock::new();

pub fn send_hamlib_command(cmd: impl FnOnce(&ConfigCommands) -> &[String]) -> Result<()> {
    let Some(commander) = HAMLIB_COMMANDER.get() else {
        return Ok(());
    };
    let Some(commands) = COMMANDS.get().map(cmd) else {
        return Ok(());
    };
    if commands.is_empty() {
        return Ok(());
    }
    let mut locked = commander.lock().expect("lock must be obtained");
    locked.send(commands)?;
    Ok(())
}

pub fn close_commander() -> Result<()> {
    let Some(commander) = HAMLIB_COMMANDER.get() else {
        return Ok(());
    };
    let mut locked = commander.lock().expect("lock must be obtained");
    locked.close()?;
    Ok(())
}

pub fn operate_fsk<T>(op: impl FnOnce(&mut SpinFsk) -> T) -> Result<Option<T>> {
    let Some(spin_fsk) = SPIN_FSK.get() else {
        return Ok(None);
    };
    let mut locked = spin_fsk.lock().expect("lock must be obtained");
    Ok(Some(op(&mut locked)))
}

pub fn close_fsk() {
    let Some(spin_fsk) = SPIN_FSK.get() else {
        return;
    };
    let mut locked = spin_fsk.lock().expect("lock must be obtained");
    locked.close();
}

fn initialize_ptt(commander: Box<dyn HamlibCommander>, commands: ConfigCommands) {
    HAMLIB_COMMANDER.set(Mutex::new(commander)).ok();
    COMMANDS.set(commands).ok();
}

fn initialize_fsk(spin_fsk: SpinFsk) {
    SPIN_FSK.set(Mutex::new(spin_fsk)).ok();
}
