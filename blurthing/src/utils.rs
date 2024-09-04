use ::image::DynamicImage;
use anyhow::Result;

pub fn download_random_image() -> Result<DynamicImage> {
    let bytes = reqwest::blocking::get("https://picsum.photos/512")?.bytes()?;
    ::image::load_from_memory(&bytes).map_err(Into::into)
}
