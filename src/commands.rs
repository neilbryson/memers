use clap::{Parser, Subcommand};

/// Meme generator
#[derive(Parser, Debug)]
#[clap(name = "memers")]
#[clap(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: CommandTypes,
}

#[derive(Subcommand, Debug)]
pub enum CommandTypes {
    /// Drake No / Yes meme
    Drake {
        #[clap(short, long)]
        top_text: String,
        #[clap(short, long)]
        bottom_text: String,
        /// This puts Drake on the right-hand side
        #[clap(short, long)]
        inverted: Option<bool>,
    },
}

pub struct Commands;

impl Commands {
    pub fn run(command: CommandTypes) {
        match command {
            CommandTypes::Drake {
                top_text,
                bottom_text,
                inverted,
            } => {
                todo!();
            }
        }
    }
}
