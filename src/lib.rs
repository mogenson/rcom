use tokio::io::{split, stdin, stdout, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::task::spawn_local;
use tokio_serial::{DataBits, FlowControl, Parity, Serial, SerialPortSettings, StopBits};

static SETTINGS: SerialPortSettings = SerialPortSettings {
    baud_rate: 115_200,
    data_bits: DataBits::Eight,
    flow_control: FlowControl::None,
    parity: Parity::None,
    stop_bits: StopBits::One,
    timeout: std::time::Duration::from_secs(0),
};

pub async fn runner(port_name: &str) -> Result<(), std::io::Error> {
    let port = Serial::from_path(port_name, &SETTINGS)
        .expect(format!("\tUnable to open serial port: {}", port_name).as_str());
    println!("\tConnected to serial port: {}", port_name);

    let (reader, mut writer) = split(port);

    let read = spawn_local(async {
        let mut lines = BufReader::new(reader).lines();
        let mut stdout = stdout();
        while let Some(line) = lines.next_line().await? {
            stdout.write_all(format!("{}\n", line).as_bytes()).await?;
        }

        Ok(()) as Result<(), std::io::Error>
    });

    let write = spawn_local(async move {
        let mut lines = BufReader::new(stdin()).lines();
        while let Some(line) = lines.next_line().await? {
            writer.write_all(format!("{}\n", line).as_bytes()).await?;
        }

        Ok(()) as Result<(), std::io::Error>
    });

    read.await??;
    write.await??;

    Ok(())
}
