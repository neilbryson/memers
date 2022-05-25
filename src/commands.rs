use crate::memes::{generate_drake, generate_fan_vs_enjoyer, generate_gigachad};
use anyhow::Result;
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
        top_text: String,
        bottom_text: String,
        /// Where to save the image
        output_path: String,
    },
    /// Average Fan vs. Average GigaChad Enjoyer
    Fve {
        /// Text to insert above the Average Fan
        fan_text: String,
        /// Text to insert above the glorious GigaChad
        enjoyer_text: String,
        /// Where to save the image
        output_path: String,
    },
    /// No description necessary.
    Gigachad {
        text: String,
        /// Where to save the image
        output_path: String,
    },
}

pub fn run_command(command: CommandTypes) -> Result<()> {
    match command {
        CommandTypes::Drake {
            top_text,
            bottom_text,
            output_path,
        } => generate_drake(top_text, bottom_text, output_path),
        CommandTypes::Fve {
            fan_text,
            enjoyer_text,
            output_path,
        } => generate_fan_vs_enjoyer(fan_text, enjoyer_text, output_path),
        CommandTypes::Gigachad { text, output_path } => generate_gigachad(text, output_path),
    }
}
