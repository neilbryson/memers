use crate::image_process::{create_text_img, load_image, paste_image, process_gif};
use crate::utils::{load_font, print_output_path};
use anyhow::Result;
use image::{codecs::gif, RgbaImage};
use std::fs;

pub fn generate_drake(top_text: String, bottom_text: String, output_path: String) -> Result<()> {
    println!("Generating Drake meme...");

    let mut output_img = RgbaImage::new(800, 800);
    let drake_no_img = load_image("img/drake-no.jpg", 400)?;
    let drake_yes_img = load_image("img/drake-yes.jpeg", 400)?;

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
pub fn generate_fan_vs_enjoyer(
    fan_text: String,
    enjoyer_text: String,
    output_path: String,
) -> Result<()> {
    println!("Generating Average Fan vs. Average GigaChad Enjoyer meme.");

    let font = load_font().unwrap();

    println!("Decoding Average Fan gif");
    let average_fan_frames = process_gif("img/average-fan.gif", fan_text.as_str(), &font)?;

    println!("Decoding GigaChad gif");
    let gigachad_frames = process_gif("img/gigachad.gif", enjoyer_text.as_str(), &font)?;

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

pub fn generate_gigachad(text: String, output_path: String) -> Result<()> {
    println!("Generating GigaChad meme.");

    let font = load_font().unwrap();

    println!("Decoding GigaChad gif");
    let gigachad_frames = process_gif("img/gigachad.gif", text.as_str(), &font)?;

    println!("Creating file");
    let output_file = fs::File::create(&output_path)?;
    let mut encoder = gif::GifEncoder::new_with_speed(output_file, 28);
    encoder.set_repeat(gif::Repeat::Infinite)?;
    encoder.encode_frames(gigachad_frames)?;

    print_output_path(&output_path)?;

    Ok(())
}
