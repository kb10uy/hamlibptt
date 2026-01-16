use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU8, Ordering},
    },
    thread::{JoinHandle, spawn},
    time::Duration,
};

use serialport::{COMPort, Error as SerialPortError, SerialPort};
use spin_sleep::SpinSleeper;

use crate::core::show_error_dialog;

#[derive(Debug, Clone)]
pub struct FskParameter {
    pub data_bits: usize,
    pub baud: f64,
    pub stop_bit: FskStopbit,
    pub target: FskTarget,
    pub invert: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FskTarget {
    Dtr,
    Rts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum FskStopbit {
    One,
    OneHalf,
    Two,
}

#[derive(Debug)]
pub struct SpinFsk {
    busy: Arc<AtomicBool>,
    buffer: Arc<AtomicU8>,
    closing: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl SpinFsk {
    pub fn start(device: &str, parameter: FskParameter) -> Result<SpinFsk, SerialPortError> {
        let busy = Arc::new(AtomicBool::new(false));
        let buffer = Arc::new(AtomicU8::new(0));
        let closing = Arc::new(AtomicBool::new(false));

        let port = serialport::new(device, 9600).open_native()?;
        let rx_buffer = buffer.clone();
        let rx_closing = closing.clone();
        let rx_busy = busy.clone();
        let thread = spawn(move || run(port, parameter, rx_buffer, rx_closing, rx_busy));

        Ok(SpinFsk {
            busy,
            buffer,
            closing,
            thread: Some(thread),
        })
    }

    pub fn is_busy(&self) -> bool {
        self.busy.load(Ordering::Acquire)
    }

    pub fn send(&self, byte: u8) {
        if self.busy.swap(true, Ordering::AcqRel) {
            return;
        }
        self.buffer.store(byte, Ordering::Release);
    }

    pub fn close(&mut self) {
        let Some(thread) = self.thread.take() else {
            return;
        };

        self.closing.store(true, Ordering::Release);
        thread.join().expect("FSK thread panicked");
    }
}

impl Drop for SpinFsk {
    fn drop(&mut self) {
        self.close();
    }
}

fn run(
    port: COMPort,
    parameter: FskParameter,
    buffer: Arc<AtomicU8>,
    closing: Arc<AtomicBool>,
    busy: Arc<AtomicBool>,
) {
    match run_inner(port, parameter, buffer, closing, busy) {
        Ok(()) => (),
        Err(e) => {
            show_error_dialog(&format!("FSK error: {e}"));
        }
    }
}

fn run_inner(
    mut port: COMPort,
    parameter: FskParameter,
    buffer: Arc<AtomicU8>,
    closing: Arc<AtomicBool>,
    busy: Arc<AtomicBool>,
) -> Result<(), SerialPortError> {
    let mut set_fsk = move |bit: bool| match parameter.target {
        FskTarget::Dtr => port.write_data_terminal_ready(bit ^ parameter.invert),
        FskTarget::Rts => port.write_request_to_send(bit ^ parameter.invert),
    };

    // 10us accuracy
    let sleeper = SpinSleeper::new(10_000);
    let half_bit_tick = Duration::from_secs_f64(1.0 / parameter.baud / 2.0);
    let stop_bit_tick = match parameter.stop_bit {
        FskStopbit::One => half_bit_tick * 2,
        FskStopbit::OneHalf => half_bit_tick * 3,
        FskStopbit::Two => half_bit_tick * 4,
    };

    busy.store(false, Ordering::Release);
    set_fsk(true)?;
    'wait_data: while !closing.load(Ordering::Acquire) {
        let byte = buffer.load(Ordering::Acquire);
        if let Ok(true) = busy.compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed) {
            sleeper.sleep(half_bit_tick);
            continue 'wait_data;
        }

        // start
        set_fsk(false)?;
        sleeper.sleep(half_bit_tick * 2);
        // data
        for b in 0..parameter.data_bits {
            let bit = (byte >> b) & 1;
            set_fsk(bit != 0)?;
            sleeper.sleep(half_bit_tick * 2);
        }
        // stop
        set_fsk(true)?;
        sleeper.sleep(stop_bit_tick);
    }
    Ok(())
}
