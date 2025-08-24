use wasm_bindgen::prelude::wasm_bindgen;

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
    use image::{GenericImageView, Pixel};
    use std::process;

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
