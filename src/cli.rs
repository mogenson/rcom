use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub const PORT: &str = "PORT";
#[cfg(unix)]
pub const DEFAULT_PORT: &str = "/dev/ttyUSBC-DEBUG";
#[cfg(windows)]
pub const DEFAULT_PORT: &str = "COM1";

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name(PORT)
                .help("Sets serial port")
                .takes_value(true)
                .short("p")
                .long("port")
                .default_value(DEFAULT_PORT),
        )
}
