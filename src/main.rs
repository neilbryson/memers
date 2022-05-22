mod commands;
mod image;
mod utils;

use crate::commands::{run_command, Cli};
use clap::Parser;

fn main() {
    let args: Cli = Cli::parse();
    run_command(args.command).unwrap_or_else(|err| {
        println!("Error: {}", err.to_string());
    });
}
