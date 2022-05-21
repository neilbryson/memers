use crate::image::{create_text_img, load_image, paste_image};
use anyhow::Result;
use clap::{Parser, Subcommand};
use image::RgbaImage;
use std::fs;

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
        } => {
            println!("Generating Drake meme...");

            let mut output_img = RgbaImage::new(800, 800);
            let drake_no_img = load_image("src/img/drake-no.jpg", 400)?;
            let drake_yes_img = load_image("src/img/drake-yes.jpeg", 400)?;

            paste_image(&mut output_img, &drake_no_img, (0, 0));
            paste_image(&mut output_img, &drake_yes_img, (0, 400));

            let top_text_img = create_text_img(400, 64.0, top_text.as_str())?;
            let bottom_text_img = create_text_img(400, 64.0, bottom_text.as_str())?;

            paste_image(&mut output_img, &top_text_img, (400, 0));
            paste_image(&mut output_img, &bottom_text_img, (400, 400));

            output_img.save(&output_path)?;
            let canonical_path = fs::canonicalize(&output_path)?;
            println!("Image created at {}", &canonical_path.to_str().unwrap());

            Ok(())
        }
    }
}
