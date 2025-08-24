use image::{GenericImageView, Pixel};
use std::process;
use std::{error::Error, fs};

use wasm_bindgen::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./image.png";
    let img_bytes = fs::read(file_path)?;

    let image = get_image(img_bytes.as_slice());
    assert_eq!(image.width(), 800);
    assert_eq!(image.height(), 600);

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
pub fn get_image(image_bytes: &[u8]) -> Image {
    let img = image::load_from_memory(image_bytes).unwrap_or_else(|_| process::abort());
    let data = img
        .pixels()
        .map(|p| p.2.channels().to_vec())
        .collect::<Vec<Vec<u8>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();

    Image::new(img.width(), img.height(), data)
}
