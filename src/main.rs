use tokio::runtime::Runtime;
use tokio::task::LocalSet;

mod cli;
mod lib;

fn main() -> Result<(), std::io::Error> {
    let args = cli::build_cli().get_matches();

    let port_name = args.value_of(cli::PORT).unwrap();

    println!("Welcome to the Root Robot Communication Terminal (RCOM)");
    let mut rt = Runtime::new()?;
    let local = LocalSet::new();
    local.block_on(&mut rt, lib::runner(port_name))
}
