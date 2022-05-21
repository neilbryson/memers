use anyhow::Result;
use image::imageops::FilterType;
use image::{io::Reader as ImageReader, DynamicImage, GenericImage, ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

pub fn load_image(path: &str, to_size: u32) -> Result<DynamicImage> {
    let img = ImageReader::open(path)?
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
    let font_vec = Vec::from(include_bytes!("fonts/BebasNeue-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font_vec).unwrap();
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
