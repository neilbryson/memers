use crate::assets;
use crate::utils::load_font;
use anyhow::Result;
use image::imageops::FilterType;
use image::{
    codecs::gif::GifDecoder, imageops, io::Reader as ImageReader, AnimationDecoder, DynamicImage,
    Frame, GenericImage, ImageBuffer, Rgba, RgbaImage,
};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::io::Cursor;

pub fn load_image(asset_name: &str, to_size: u32) -> Result<DynamicImage> {
    let img_asset = assets::Images::get(asset_name).unwrap();
    let img = ImageReader::new(Cursor::new(img_asset.data))
        .with_guessed_format()?
        .decode()?
        .resize(to_size, to_size, FilterType::Nearest);
    Ok(img)
}

pub fn paste_image<I>(dest_buf: &mut RgbaImage, img_to_paste: &I, starting_px: (u32, u32))
where
    I: GenericImage<Pixel = Rgba<u8>>,
{
    for (x, y, rgba) in img_to_paste.pixels() {
        let to_x = x + starting_px.0;
        let to_y = y + starting_px.1;
        dest_buf.put_pixel(to_x, to_y, rgba);
    }
}

pub fn create_text_img(size: u32, font_size: f32, text: &str) -> Result<RgbaImage> {
    let mut img: RgbaImage =
        ImageBuffer::from_fn(size, size, |_, _| Rgba([255u8, 255u8, 255u8, 255u8]));
    let font = load_font().unwrap();
    let scale = Scale {
        x: font_size,
        y: font_size,
    };
    draw_text_mut(
        &mut img,
        Rgba([0u8, 0u8, 0u8, 255u8]),
        0,
        0,
        scale,
        &font,
        text,
    );

    Ok(img)
}

pub fn insert_text_frame(
    dest_vec: &mut Vec<Frame>,
    frame_buf: &RgbaImage,
    font: &Font,
    font_size: f32,
    width: u32,
    height: u32,
    text: &str,
) {
    let scale = Scale {
        x: font_size,
        y: font_size,
    };
    let mut res = imageops::resize(&*frame_buf, width, height, FilterType::Nearest);
    draw_text_mut(
        &mut res,
        Rgba([0xFF, 0xFF, 0xFF, 0xFF]),
        0,
        0,
        scale,
        &font,
        text,
    );
    let frm = Frame::new(res);
    dest_vec.push(frm);
}

pub fn process_gif(asset_name: &str, text_to_insert: &str, font: &Font) -> Result<Vec<Frame>> {
    let gif_file = assets::Images::get(asset_name).unwrap();
    let decoded = GifDecoder::new(Cursor::new(gif_file.data))?.into_frames();
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
