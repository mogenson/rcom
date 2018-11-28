extern crate serialport;

use serialport::prelude::*;
use std::env;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::thread;
use std::time::Duration;

/* serial port path */
#[cfg(target_os = "linux")]
const PATH: &str = "/dev/ttyUSBC-DEBUG";
#[cfg(target_os = "macos")]
const PATH: &str = "/dev/cu.SLAB_USBtoUART";
#[cfg(target_os = "windows")]
const PATH: &str = "COM1";

/* serial port settings */
static SETTINGS: SerialPortSettings = SerialPortSettings {
    baud_rate: 115_200,
    data_bits: DataBits::Eight,
    flow_control: FlowControl::None,
    parity: Parity::None,
    stop_bits: StopBits::One,
    timeout: Duration::from_secs(1),
};

fn main() {
    /* accept serial port path or use default */
    let mut args = env::args();
    let path = args.nth(1).unwrap_or_else(|| PATH.into());

    /* print help */
    println!("Welcome to the Root Robot Communication Terminal (RCOM)");
    if !Path::new(&path).exists() {
        println!("Usage: rcom [/path/to/serialport]");
        process::exit(0);
    }

    /* open port with settings */
    let writer_port = serialport::open_with_settings(&path, &SETTINGS);
    match writer_port {
        Ok(_) => println!("Connected to serial port: {}", path),
        Err(error) => {
            println!("Failed to connect to serial port: {}", path);
            println!("With error: {}", error);
            process::exit(0);
        }
    };
    let mut writer_port = writer_port.unwrap();

    /* start thread to read and print from serial port, byte by byte */
    let reader_port = writer_port.try_clone().expect("can't clone serial port");
    thread::spawn(move || {
        for byte in reader_port.bytes() {
            if let Ok(b) = byte {
                if b == 0xA {
                    println!(); // print new line if byte is \n
                } else {
                    print!("{}", b as char); // or print byte as char
                }
            }
        }
    });

    /* read from stdin, line by line */
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            writer_port
                .write_all(format!("{}\n", l).as_bytes())
                .expect("can't write to serial port");
        }
    }
}
