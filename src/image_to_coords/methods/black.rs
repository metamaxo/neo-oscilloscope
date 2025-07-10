use crate::image_to_coords::request::Request;

use image::GrayImage;

impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    fn is_black_interpolated(&self, x: f32, y: f32) -> bool {
        let ix = x.round().clamp(0.0, (self.size - 1) as f32) as u32;
        let iy = y.round().clamp(0.0, (self.size - 1) as f32) as u32;
        if ix >= self.size || iy >= self.size {
            return false;
        }
        self.is_black(ix, iy)
    }

    pub fn trace_black_coords(&self) -> Vec<(u32, u32)> {
        let upscale = 4;
        let mut high_res_coords = Vec::new();

        let width = self.size - 1;
        let height = self.size - 1;

        for y in 0..(height * upscale) {
            for x in 0..(width * upscale) {
                // Map high-res (x, y) back to low-res image space
                let low_x = x as f32 / upscale as f32;
                let low_y = y as f32 / upscale as f32;
                // Use interpolation or nearest-neighbor sampling
                if self.is_black_interpolated(low_x, low_y) {
                    high_res_coords.push((low_x, low_y));
                }
            }
        }

        // Optional: Deduplicate (round to nearest 1/upscale pixel)
        let mut unique = std::collections::HashSet::new();
        let mut result = Vec::new();
        for (x, y) in high_res_coords {
            let key = (
                (x * upscale as f32).round() as u32,
                (y * upscale as f32).round() as u32,
            );
            if unique.insert(key) {
                result.push((x as u32, y as u32));
            }
        }

        result
    }
}
