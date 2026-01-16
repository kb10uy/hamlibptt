use std::path::Path;

use crate::{
    core::{
        config::{ConfigControlMode, ConfigFskTarget, load_config},
        error::{HamlibPttError, Result},
        show_info_dialog,
    },
    extfsk::{
        close_commander, close_fsk, initialize_fsk, initialize_ptt,
        parameter::{ExtfskParameter, ExtfskStopbit},
        send_hamlib_command,
    },
    hamlib::{
        HamlibCommander,
        rigctl::{RetainedRigctlCommander, RigctlCommander},
        rigctld::{RetainedRigctldCommander, RigctldCommander},
    },
    spinfsk::{FskParameter, FskStopbit, FskTarget, SpinFsk},
};

pub fn open(dll_directory: &Path, extfsk_parameter: ExtfskParameter) -> Result<()> {
    let config = load_config(dll_directory)?;

    let (commander, ptt_message): (Box<dyn HamlibCommander>, _) = match config.mode {
        ConfigControlMode::Rigctl => {
            let Some(rigctl) = &config.rigctl else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            (
                Box::new(RigctlCommander::new(rigctl)),
                format!(
                    "rigctl at {} (id={}, device={})",
                    rigctl.rigctl_path, rigctl.model_id, rigctl.device
                ),
            )
        }
        ConfigControlMode::RigctlRetained => {
            let Some(rigctl) = &config.rigctl else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            (
                Box::new(RetainedRigctlCommander::new(rigctl)?),
                format!(
                    "retained connection rigctl at {} (id={}, device={})",
                    rigctl.rigctl_path, rigctl.model_id, rigctl.device
                ),
            )
        }
        ConfigControlMode::Rigctld => {
            let Some(rigctld) = &config.rigctld else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            (
                Box::new(RigctldCommander::new(rigctld)),
                format!("rigctld on {}", rigctld.address),
            )
        }
        ConfigControlMode::RigctldRetained => {
            let Some(rigctld) = &config.rigctld else {
                return Err(HamlibPttError::ConfigDataInvalid);
            };
            (
                Box::new(RetainedRigctldCommander::new(rigctld)?),
                format!("retained connection rigctld on {}", rigctld.address),
            )
        }
    };

    let fsk = match (config.enable_fsk, config.fsk) {
        (Some(true), Some(fsk)) => Some(fsk),
        (Some(true), None) => return Err(HamlibPttError::ConfigDataInvalid),
        _ => None,
    };

    initialize_ptt(commander, config.commands.clone());
    send_hamlib_command(|cmds| cmds.open.as_deref().unwrap_or_default())?;

    let fsk_message = match fsk {
        Some(fsk) => {
            let parameter = FskParameter {
                data_bits: extfsk_parameter.length as usize,
                baud: extfsk_parameter.baud as f64,
                stop_bit: match extfsk_parameter.stop_bit {
                    ExtfskStopbit::One => FskStopbit::One,
                    ExtfskStopbit::OneHalf => FskStopbit::OneHalf,
                    ExtfskStopbit::Two => FskStopbit::Two,
                },
                target: match fsk.target {
                    ConfigFskTarget::Dtr => FskTarget::Dtr,
                    ConfigFskTarget::Rts => FskTarget::Rts,
                },
                invert: fsk.invert.unwrap_or(false),
            };
            let message = format!("enabled on {} ({})", fsk.device, parameter);

            let spin_fsk = SpinFsk::start(&fsk.device, parameter)?;
            initialize_fsk(spin_fsk);
            message
        }
        None => "disabled".to_string(),
    };

    show_info_dialog(&format!(
        "Initialized successfully!\n\nPTT: {ptt_message}\n\nFSK: {fsk_message}"
    ));

    Ok(())
}

pub fn close() -> Result<()> {
    send_hamlib_command(|cmds| cmds.close.as_deref().unwrap_or_default())?;
    close_commander()?;
    close_fsk();
    Ok(())
}
