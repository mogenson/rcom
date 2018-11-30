extern crate serialport;

#[cfg(unix)]
use serialport::posix::TTYPort;
use serialport::prelude::*;
use serialport::SerialPortType;
use std::env;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

/* silicon labs usb serial cable */
static VID: u16 = 0x10C4;
static PID: u16 = 0xEA60;

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
    /* accept serial port path or find one */
    let first_arg = env::args().nth(1);
    let path = match first_arg {
        Some(path) => PathBuf::from(path),
        None => {
            let mut path = PathBuf::new();
            if let Ok(ports) = serialport::available_ports() {
                for port in ports {
                    match port.port_type {
                        SerialPortType::UsbPort(info) => {
                            /* found silicon labs usb serial port */
                            if info.vid == VID && info.pid == PID {
                                path.push(port.port_name);
                            }
                        }
                        _ => (),
                    }
                }
            }
            path
        }
    };

    /* print help */
    println!("Welcome to the Root Robot Communication Terminal (RCOM)");
    if !path.exists() {
        println!("\tNo serial port found");
        println!("\tUsage: rcom [/path/to/serialport]");
        process::exit(0);
    }

    /* open port */
    #[cfg(unix)]
    let writer_port = TTYPort::open(&path, &SETTINGS);
    #[cfg(windows)]
    let writer_port = serialport::open_with_settings(&path, &SETTINGS);

    match writer_port {
        Ok(_) => println!(
            "\tConnected to serial port: {}",
            path.to_str().unwrap_or("None")
        ),
        Err(error) => {
            println!(
                "\tFailed to connect to serial port: {}",
                path.to_str().unwrap_or("None")
            );
            println!("\tWith error: {}", error);
            process::exit(0);
        }
    };
    let mut writer_port = writer_port.unwrap();

    /* remove exclusive access */
    #[cfg(unix)]
    match writer_port.set_exclusive(false) {
        Ok(_) => (),
        Err(error) => println!("Error setting non-exclusive: {}", error),
    };

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
