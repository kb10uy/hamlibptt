use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, RecvTimeoutError, SendError, Sender, TryRecvError, channel},
    },
    thread::{JoinHandle, spawn},
    time::Duration,
};

use serialport::{COMPort, Error as SerialPortError, SerialPort};
use spin_sleep::SpinSleeper;

use crate::core::show_error_dialog;

const RX_TIMEOUT: Duration = Duration::from_secs(1);

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
    tx: Sender<u8>,
    close: Sender<()>,
    thread: Option<JoinHandle<()>>,
}

impl SpinFsk {
    pub fn start(device: &str, parameter: FskParameter) -> Result<SpinFsk, SerialPortError> {
        let busy = Arc::new(AtomicBool::new(false));
        let (tx, data_rx) = channel();
        let (close, close_rx) = channel();

        let port = serialport::new(device, 9600).open_native()?;
        let busy_rx = busy.clone();
        let thread = spawn(move || run(port, parameter, data_rx, close_rx, busy_rx));

        Ok(SpinFsk {
            busy,
            tx,
            close,
            thread: Some(thread),
        })
    }

    pub fn is_busy(&self) -> bool {
        self.busy.load(Ordering::Acquire)
    }

    pub fn send(&self, byte: u8) -> Result<(), SendError<u8>> {
        self.tx.send(byte)
    }

    pub fn close(&mut self) {
        let Some(thread) = self.thread.take() else {
            return;
        };

        self.close.send(()).expect("close must be sent");
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
    rx: Receiver<u8>,
    close: Receiver<()>,
    busy: Arc<AtomicBool>,
) {
    match run_inner(port, parameter, rx, close, busy) {
        Ok(()) => (),
        Err(e) => {
            show_error_dialog(&format!("FSK error: {e}"));
        }
    }
}

fn run_inner(
    mut port: COMPort,
    parameter: FskParameter,
    rx: Receiver<u8>,
    close: Receiver<()>,
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
    while let Err(TryRecvError::Empty) = close.try_recv() {
        let byte = match rx.recv_timeout(RX_TIMEOUT) {
            Ok(b) => b,
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => unreachable!("must not be disconnected"),
        };
        busy.store(true, Ordering::Release);

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

        busy.store(false, Ordering::Release);
    }
    Ok(())
}
