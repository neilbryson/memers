use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/img/"]
#[prefix = "img/"]
pub struct Images;
