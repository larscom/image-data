use image::{GenericImageView, Pixel};
use std::error::Error;

use wasm_bindgen::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./image.png";
    let img = image::open(file_path)?;

    Ok(())
}

#[wasm_bindgen]
pub struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl Image {
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Image {
        Image {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[wasm_bindgen]
pub fn get_pixels(image_bytes: &[u8]) -> Image {
    let img = image::load_from_memory(image_bytes).expect("failed to load image");
    let data = img
        .pixels()
        .map(|p| p.2.channels().to_vec())
        .collect::<Vec<Vec<u8>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();

    Image::new(img.width(), img.height(), data)
}
