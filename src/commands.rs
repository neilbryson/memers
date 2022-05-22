use crate::image::{create_text_img, insert_text_frame, load_image, paste_image};
use crate::utils::{load_font, print_output_path};
use anyhow::Result;
use clap::{Parser, Subcommand};
use image::{codecs::gif, AnimationDecoder, Frame, RgbaImage};
use rusttype::Font;
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

fn generate_drake(top_text: String, bottom_text: String, output_path: String) -> Result<()> {
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
    print_output_path(&output_path)?;

    Ok(())
}

// code still dirty. will probably improve this some other time.. hopefully
fn generate_fan_vs_enjoyer(
    fan_text: String,
    enjoyer_text: String,
    output_path: String,
) -> Result<()> {
    println!("Generating Average Fan vs. Average GigaChad Enjoyer meme.");

    let font = load_font().unwrap();

    println!("Decoding Average Fan gif");
    let average_fan_frames = process_gif("src/img/average-fan.gif", fan_text.as_str(), &font)?;

    println!("Decoding GigaChad gif");
    let gigachad_frames = process_gif("src/img/gigachad.gif", enjoyer_text.as_str(), &font)?;

    println!("Creating file...\nThis might take a long time.");
    let output_file = fs::File::create(&output_path)?;
    let mut encoder = gif::GifEncoder::new_with_speed(output_file, 28);
    encoder.set_repeat(gif::Repeat::Infinite)?;

    println!("Appending processed Average Fan gif");
    encoder.encode_frames(average_fan_frames)?;

    println!("Appending processed GigaChad gif");
    encoder.encode_frames(gigachad_frames)?;

    print_output_path(&output_path)?;

    Ok(())
}

fn generate_gigachad(text: String, output_path: String) -> Result<()> {
    println!("Generating GigaChad meme.");

    let font = load_font().unwrap();

    println!("Decoding GigaChad gif");
    let gigachad_frames = process_gif("src/img/gigachad.gif", text.as_str(), &font)?;

    println!("Creating file");
    let output_file = fs::File::create(&output_path)?;
    let mut encoder = gif::GifEncoder::new_with_speed(output_file, 28);
    encoder.set_repeat(gif::Repeat::Infinite)?;
    encoder.encode_frames(gigachad_frames)?;

    print_output_path(&output_path)?;

    Ok(())
}

fn process_gif(path: &str, text_to_insert: &str, font: &Font) -> Result<Vec<Frame>> {
    let gif_file = fs::File::open(path)?;
    let decoded = gif::GifDecoder::new(gif_file)?.into_frames();
    let raw_frames = decoded.collect_frames().expect("error decoding gif");

    let mut processed_frames: Vec<Frame> = Vec::new();

    for frame in &raw_frames {
        let frm_buf = frame.buffer();
        insert_text_frame(
            &mut processed_frames,
            &frm_buf,
            &font,
            64.0,
            360,
            640,
            text_to_insert,
        );
    }

    Ok(processed_frames)
}
