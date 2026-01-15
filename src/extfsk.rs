pub mod core;
pub mod fsk;
pub mod parameter;
pub mod ptt;

use std::sync::{Mutex, OnceLock};

use crate::{
    core::{config::ConfigCommands, error::Result},
    hamlib::HamlibCommander,
};

static COMMANDER: OnceLock<Mutex<Box<dyn HamlibCommander>>> = OnceLock::new();
static COMMANDS: OnceLock<ConfigCommands> = OnceLock::new();


pub fn run_command(cmd: impl FnOnce(&ConfigCommands) -> &[String]) -> Result<()> {
    let Some(commander) = COMMANDER.get() else {
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

fn initialize_backend(commander: Box<dyn HamlibCommander>, commands: ConfigCommands) {
    COMMANDER.set(Mutex::new(commander)).ok();
    COMMANDS.set(commands).ok();
}
