use crate::image_to_coords::request::Request;
use image::GrayImage;

pub const MOORE_DIRS: [(i32, i32); 8] = [
    (0, -1),  // N
    (1, -1),  // NE
    (1, 0),   // E
    (1, 1),   // SE
    (0, 1),   // S
    (-1, 1),  // SW
    (-1, 0),  // W
    (-1, -1), // NW
];

impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    #[inline(always)]
    pub fn is_black(&self, x: u32, y: u32) -> bool {
        if let Some(pixel) = self.image.get_pixel_checked(x, y) {
            pixel[0] < self.threshold
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn is_edge(&self, x: u32, y: u32) -> bool {
        if !self.is_black(x, y) {
            return false;
        }
        for (dx, dy) in &MOORE_DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && (nx as u32) < self.size
                && (ny as u32) < self.size
                && self.image.get_pixel(nx as u32, ny as u32)[0] >= self.threshold
            {
                return true;
            }
        }
        false
    }

    pub fn check_pixel(&self, x: u32, y: u32) -> bool {
        match self.edge_detection {
            true => self.is_edge(x, y),
            false => self.is_black(x, y),
        }
    }

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
