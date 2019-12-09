use clap::{crate_name, Shell};

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    let mut app = cli::build_cli();
    app.gen_completions(crate_name!(), Shell::Bash, outdir);
}
