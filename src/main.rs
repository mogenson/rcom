use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio_serial::{DataBits, FlowControl, Parity, Serial, SerialPortSettings, StopBits};

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSBC-DEBUG";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

static SETTINGS: SerialPortSettings = SerialPortSettings {
    baud_rate: 115_200,
    data_bits: DataBits::Eight,
    flow_control: FlowControl::None,
    parity: Parity::None,
    stop_bits: StopBits::One,
    timeout: std::time::Duration::from_secs(0),
};

async fn runner() -> Result<(), std::io::Error> {
    let tty_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_TTY.into());
    let port = Serial::from_path(tty_path, &SETTINGS).expect("Unable to open serial port");
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
