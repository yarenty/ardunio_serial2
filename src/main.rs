use std::borrow::BorrowMut;
use std::cell::Cell;
use std::io;
use std::io::Read;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use chrono::prelude::*;
use log::{debug, error, info, trace, warn};
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};

use utils::setup_logger;

use crate::errors::Result;

mod errors;
mod utils;



fn find_port() -> Result<String> {
    for i in 1..3 {
        sleep(Duration::from_millis(100));
        let ports = serialport::available_ports().expect("No ports found!");
        info!("Scanning all ports::{}/2 time.", i);
        for p in ports {
            debug!("Port::{}", p.port_name);
            let baud_rate = 9600;

            for i in 1..3 {
                sleep(Duration::from_millis(100));
                debug!("..trying {} time.", i);
                match serialport::new(&p.port_name, baud_rate).open() {
                    Err(e) => warn!("Failed to open \"{}\". Error: {}", p.port_name, e),
                    Ok(mut pr) => {
                        let mut serial_buf: Vec<u8> = vec![0; 1000];
                        debug!(
                            "Checking response on {} at {} baud:",
                            &p.port_name, &baud_rate
                        );
                        sleep(Duration::from_millis(100));

                        //try 2 times

                        match pr.read(serial_buf.as_mut_slice()) {
                            Ok(t) => {
                                let msg = String::from_utf8_lossy(&serial_buf[..t]);
                                {
                                    debug!("Got message: {:?}", msg);
                                    if msg.contains("LIGHT") {
                                        return Ok(p.port_name);
                                    }
                                }
                            }
                            Err(e) => {
                                warn!("{} :: {:?}", p.port_name, e);
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("Port not found")
}


fn get_port(name: &String, baud_rate: u32) -> Cell<Box<dyn SerialPort>> {
    match serialport::new(name, baud_rate).open() {
        Err(e) => {
            error!("Failed to open \"{}\". Error: {}", name, e);
            ::std::process::exit(1);
        }
        Ok(p) => Cell::new(p),
    }
}

fn checks(mut port: Cell<Box<dyn SerialPort>>) -> Result<()> {
    let baud_rate = 9600;
    let mut port = port.get_mut();
    baud_rate_check!(port, baud_rate);
    data_bits_check!(port, DataBits::Eight);
    flow_control_check!(port, FlowControl::None);
    parity_check!(port, Parity::None);
    stop_bits_check!(port, StopBits::One);
    Ok(())
}



fn collect_and_send(mut port: Cell<Box<dyn SerialPort>>) -> Result<()> {
    let mut sound: Vec<i32> = vec![];
    let mut light: Vec<i32> = vec![];
    let mut movement: Vec<i32> = vec![];

    let mut utc: DateTime<Utc> = Utc::now();
    let mut current_minute: u32 = utc.minute();

    debug!("Current min: {}", current_minute);

    let mut serial_buf: Vec<u8> = vec![0; 1000];
    // info!("Receiving data on {:?} ", port);
    loop {
        match port.get_mut().read(serial_buf.as_mut_slice()) {
            Ok(t) => {
// io::stdout().write_all(&serial_buf[..t]).unwrap()
                let msg = String::from_utf8(serial_buf[..t].to_vec())?;
                trace!("{:?}", msg);
// process add to vec
                for m in msg.split("\r\n") {
                    trace!("m:{}", m);
                    if m.len() > 10 {
                        let m = m.to_string();
                        let reading = m.split(';').collect::<Vec<&str>>();
                        trace!("r:{:?}", &reading);

                        if reading.len() == 3 {
                            if reading[0].contains(':') {
                                match &reading[0].split(':').collect::<Vec<&str>>()[1].parse() {
                                    Ok(l) => {
                                        trace!("l:{}", l);
                                        light.push(*l);
                                    }
                                    Err(e) => debug!("{:?}", e),
                                }
                            }
                            if reading[1].contains(':') {
                                match &reading[1].split(':').collect::<Vec<&str>>()[1]
                                    .parse::<i32>()
                                {
                                    Ok(o) => {
                                        trace!("o:{}", o);
                                        movement.push(*o);
                                    }
                                    Err(e) => debug!("{:?}", e),
                                }
                            }

                            if reading[2].contains(':') {
                                match &reading[2].split(':').collect::<Vec<&str>>()[1].parse() {
                                    Ok(s) => {
                                        trace!("s:{}", s);
                                        sound.push(*s);
                                    }
                                    Err(e) => debug!("{:?}", e),
                                }
                            }
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => {
                warn!("Connection error: {:?}, restarting in 5 sec..", e);
                sleep(Duration::from_millis(5000));
// exit(1);
                port = get_port(&find_port()?, 9600);
            }
        }
        sleep(Duration::from_millis(1000));
        utc = Utc::now();

        if current_minute != utc.minute() {
            let len = light.len() as i32;
            if len != 0 {
//light is iversed - max value is 1024
                let l = 1024 - light.iter().sum::<i32>() / len;
//sound is tricky as it need to accumulate over time aka. sound levels
                let s = sound.iter().max().ok_or(0).expect("Sound");
//movement is binary really
                let m = movement.iter().max().ok_or(0).expect("Motion");
//send message form vecs
                send(utc, l, *s, *m);
            }
//clean vecs
            light = vec![];
            sound = vec![];
            movement = vec![];
// next min
            current_minute = utc.minute();
        }
    }
}



fn send(date: DateTime<Utc>, light: i32, sound: i32, motion: i32) {
    let send_me = format!("http://www.yarenty.com/ardunio/add.php?year={}&month={}&day={}&hour={}&minute={}&light={}&sound={}&motion={}",
                          date.year(), date.month(), date.day(), date.hour(), date.minute(),
                          light, sound, motion
    );
    let resp = reqwest::blocking::get(&send_me);
    debug!("{} => {:?}", &send_me, resp);
    info!("{}", &send_me);
}






fn main() -> Result<()> {
    setup_logger(true, Some("info"));
    
    let name = &find_port()?;
    let baud_rate = 9600;
    
    let mut port = get_port(name, baud_rate);

    // checks(port)?;
    
    collect_and_send(port)?;
    
    Ok(())
}





