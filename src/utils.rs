use chrono::prelude::*;
use env_logger::fmt::{Color, Formatter};
use env_logger::{Builder, WriteStyle};
// use log::{debug, error, info, trace, warn};
use log::{Level, LevelFilter, Record};
// use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPort, StopBits};
// use serialport::SerialPort;
use std::io::Write;
use std::thread;

#[macro_export]
macro_rules! baud_rate_check {
    ($port:ident, $baud:expr) => {
        let baud_rate = $baud;
        if let Err(e) = $port.set_baud_rate(baud_rate) {
            error!(" Baud rate: {:?}: FAILED ({})", baud_rate, e);
        }
        match $port.baud_rate() {
            Err(_) => error!(
                "Baud rate: {:?}: FAILED (error retrieving baud rate)",
                baud_rate
            ),
            Ok(r) if r != baud_rate => error!(
                "Baud rate:  {:?}: FAILED (baud rate {:?} does not match set baud rate {:?})",
                baud_rate, r, baud_rate
            ),
            Ok(_) => info!("Baud rate: {:?} => success", baud_rate),
        }
    };
}

#[macro_export]
macro_rules! data_bits_check {
    ($port:ident, $data_bits:path) => {
        let data_bits = $data_bits;
        if let Err(e) = $port.set_data_bits(data_bits) {
            error!("Data bits: {:?}: FAILED ({})", data_bits, e);
        } else {
            match $port.data_bits() {
                Err(_) => error!("Data bits: FAILED to retrieve data bits"),
                Ok(r) if r != data_bits => error!(
                    "Data bits: {:?}: FAILED (data bits {:?} does not match set data bits {:?})",
                    data_bits, r, data_bits
                ),
                Ok(_) => info!("Data bits: {:?} => success", data_bits),
            }
        }
    };
}

#[macro_export]
macro_rules! flow_control_check {
    ($port:ident, $flow_control:path) => {
        let flow_control = $flow_control;
        if let Err(e) = $port.set_flow_control(flow_control) {
            error!("Flow control: {:?}: FAILED ({})", flow_control, e);
        } else {
            match $port.flow_control() {
                Err(_) => error!("Flow control: FAILED to retrieve flow control"),
                Ok(r) if r != flow_control => error!(
                    "Flow control: {:?}: FAILED (flow control {:?} does not match set flow control {:?})",
                    flow_control, r, flow_control
                ),
                Ok(_) => info!("Flow control: {:?} => success", flow_control),
            }
        }
    };
}

#[macro_export]
macro_rules! parity_check {
    ($port:ident, $parity:path) => {
        let parity = $parity;
        if let Err(e) = $port.set_parity(parity) {
            error!("Parity: {:?}: FAILED ({})", parity, e);
        } else {
            match $port.parity() {
                Err(_) => error!("Parity: FAILED to retrieve parity"),
                Ok(r) if r != parity => error!(
                    "Parity: {:?}: FAILED (parity {:?} does not match set parity {:?})",
                    parity, r, parity
                ),
                Ok(_) => info!("Parity: {:?} => success", parity),
            }
        }
    };
}

#[macro_export]
macro_rules! stop_bits_check {
    ($port:ident, $stop_bits:path) => {
        let stop_bits = $stop_bits;
        if let Err(e) = $port.set_stop_bits(stop_bits) {
            error!("Stop bits: {:?}: FAILED ({})", stop_bits, e);
        } else {
            match $port.stop_bits() {
                Err(_) => error!("Stop bits: FAILED to retrieve stop bits"),
                Ok(r) if r != stop_bits => error!(
                    "Stop bits: FAILED, stop bits {:?} does not match set stop bits {:?}",
                    r, stop_bits
                ),
                Ok(_) => info!("Stop bits: {:?} =>  success", stop_bits),
            }
        }
    };
}

pub fn setup_logger(log_thread: bool, rust_log: Option<&str>) {
    let output_format = move |formatter: &mut Formatter, record: &Record| {
        let thread_name = if log_thread {
            format!("(t: {}) ", thread::current().name().unwrap_or("unknown"))
        } else {
            "".to_string()
        };

        let mut thread_style = formatter.style();
        let mut level_style = formatter.style();

        match record.level() {
            Level::Error => level_style.set_color(Color::Red).set_bold(true),
            Level::Warn => level_style.set_color(Color::Red),
            Level::Info => level_style.set_color(Color::Green).set_intense(true),
            Level::Debug => level_style.set_color(Color::Blue),
            Level::Trace => level_style.set_color(Color::Magenta),
        };
        thread_style.set_color(Color::Magenta).set_intense(true);

        let local_time: DateTime<Local> = Local::now();
        let time_str = local_time.format("%H:%M:%S%.3f").to_string();
        writeln!(
            formatter,
            "{} {}{} - {} - {}",
            time_str,
            thread_style.value(thread_name),
            level_style.value(record.level()),
            record.target(),
            record.args()
        )
    };

    let mut builder = Builder::new();
    builder
        .format(output_format)
        .filter(None, LevelFilter::Info);
    builder.write_style(WriteStyle::Always);

    rust_log.map(|conf| builder.parse_filters(conf));

    builder.init();
}
