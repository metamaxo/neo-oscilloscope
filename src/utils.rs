use crate::JsInterface;
use image::{GrayImage, imageops::FilterType};
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl JsInterface {
    #[wasm_bindgen]
    pub fn generate_points(
        &mut self,
        data_l: &Float32Array,
        data_r: &Float32Array,
        scale: f32,
        center_x: f32,
        center_y: f32,
    ) -> Float32Array {
        let len_l = data_l.length();
        let len_r = data_r.length();
        let len = if len_l < len_r { len_l } else { len_r };
        let len_usize = len as usize;

        let mut result = Vec::with_capacity(len_usize * 2);

        for i in 0..len_usize {
            let xn = data_l.get_index(i as u32);
            let yn = data_r.get_index(i as u32);
            let x = center_x + xn * scale;
            let y = center_y + yn * scale;
            result.push(x);
            result.push(y);
        }

        Float32Array::from(result.as_slice())
    }
}

pub fn convert_to_canvas_size(image: &GrayImage) -> (GrayImage, u32) {
    let (height, width) = image.dimensions();
    match height > width {
        true => (
            image::imageops::resize(image, height, height, FilterType::Lanczos3),
            height,
        ),
        false => (
            image::imageops::resize(image, width, width, FilterType::Lanczos3),
            width,
        ),
    }
}
