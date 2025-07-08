use crate::image_to_coords::request::Request;
use image::GrayImage;

impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    /// Divide point map by height & width to create floats, we need these for generating audio.
    pub fn normalize(&self, outline: &[(u32, u32)]) -> Vec<(f32, f32)> {
        outline
            .iter()
            .map(|(x, y)| {
                (
                    (*x as f32 / self.size as f32) * 2.0 - 1.0, // X in [-1.0, 1.0]
                    (*y as f32 / self.size as f32) * 2.0 - 1.0, // Y in [-1.0, 1.0]
                )
            })
            .collect()
    }

    // simple interpolation for coords
    pub fn interpolate(&self, coords: &[(f32, f32)]) -> Vec<(f32, f32)> {
        let mut result = Vec::new();
        match self.interpolate {
            true => {
                for i in 0..coords.len() {
                    let (x0, y0) = coords[i];
                    let (x1, y1) = coords[(i + 1) % coords.len()];
                    for j in 0..self.int_amount {
                        let t = j as f32 / self.int_amount as f32;
                        result.push((x0 + t * (x1 - x0), y0 + t * (y1 - y0)));
                    }
                }
                result
            }
            false => coords.to_vec(),
        }
    }
}
