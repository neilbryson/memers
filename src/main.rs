mod commands;

use crate::commands::{Cli, Commands};
use clap::Parser;

fn main() {
    let args: Cli = Cli::parse();
    Commands::run(args.command);
}
