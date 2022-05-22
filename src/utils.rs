use anyhow::Result;
use rusttype::Font;
use std::fs;

pub fn load_font() -> Option<Font<'static>> {
    let font_vec = Vec::from(include_bytes!("fonts/BebasNeue-Regular.ttf") as &[u8]);
    Font::try_from_vec(font_vec)
}

pub fn print_output_path(path: &String) -> Result<()> {
    let canonical_path = fs::canonicalize(&path)?;
    println!("Image created at {}", &canonical_path.to_str().unwrap());
    Ok(())
}
