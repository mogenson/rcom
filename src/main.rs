use clap::{crate_authors, crate_description, crate_name, crate_version};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio_serial::{DataBits, FlowControl, Parity, Serial, SerialPortSettings, StopBits};

const PORT: &str = "PORT";
#[cfg(unix)]
const DEFAULT_PORT: &str = "/dev/ttyUSBC-DEBUG";
#[cfg(windows)]
const DEFAULT_PORT: &str = "COM1";

static SETTINGS: SerialPortSettings = SerialPortSettings {
    baud_rate: 115_200,
    data_bits: DataBits::Eight,
    flow_control: FlowControl::None,
    parity: Parity::None,
    stop_bits: StopBits::One,
    timeout: std::time::Duration::from_secs(0),
};

async fn runner() -> Result<(), std::io::Error> {
    let args = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            clap::Arg::with_name(PORT)
                .help("Sets serial port")
                .takes_value(true)
                .short("p")
                .long("port")
                .default_value(DEFAULT_PORT),
        )
        .get_matches();

    println!("Welcome to the Root Robot Communication Terminal (RCOM)");
    let port_name = args.value_of(PORT).unwrap();
    let port = Serial::from_path(port_name, &SETTINGS).expect("Unable to open serial port");
    println!("\tConnected to serial port: {}", port_name);

    let (reader, mut writer) = tokio::io::split(port);

    let read = tokio::task::spawn_local(async {
        let mut lines = tokio::io::BufReader::new(reader).lines();
        let mut stdout = tokio::io::stdout();
        while let Some(line) = lines.next_line().await? {
            stdout.write_all(format!("{}\n", line).as_bytes()).await?;
        }

        Ok(()) as Result<(), std::io::Error>
    });

    let write = tokio::task::spawn_local(async move {
        let mut lines = tokio::io::BufReader::new(tokio::io::stdin()).lines();
        while let Some(line) = lines.next_line().await? {
            writer.write_all(format!("{}\n", line).as_bytes()).await?;
        }

        Ok(()) as Result<(), std::io::Error>
    });

    read.await??;
    write.await??;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let mut rt = tokio::runtime::Runtime::new()?;
    let local = tokio::task::LocalSet::new();
    local.block_on(&mut rt, runner())
}
