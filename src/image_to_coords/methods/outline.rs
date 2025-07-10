use crate::image_to_coords::methods::utils::MOORE_DIRS;
use crate::image_to_coords::request::Request;

use image::GrayImage;

// Simple algorithm that finds only the outlines.
impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    pub fn trace_outline(&self) -> Vec<(u32, u32)> {
        // Find the starting black pixel
        let mut start = None;
        for y in 0..self.size {
            for x in 0..self.size {
                if self.check_pixel(x, y) {
                    start = Some((x, y));
                    break;
                }
            }
            if start.is_some() {
                break;
            }
        }

        let start = match start {
            Some(p) => p,
            None => return vec![],
        };

        let mut contour = vec![start];
        let mut current = start;
        let mut prev_dir = 0;

        let mut first_loop = true;

        loop {
            let mut found = false;
            // Start checking from (prev_dir + 6) % 8 (the pixel to the left of the last direction)
            for i in 0..8 {
                let dir_idx = (prev_dir + 6 + i) % 8;
                let (dx, dy) = MOORE_DIRS[dir_idx];
                let nx = current.0 as i32 + dx;
                let ny = current.1 as i32 + dy;

                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as u32, ny as u32);
                    if self.check_pixel(nx, ny) {
                        current = (nx, ny);
                        contour.push(current);
                        prev_dir = dir_idx;
                        found = true;
                        break;
                    }
                }
            }

            if !found || (current == start && !first_loop) {
                break;
            }

            first_loop = false;
        }

        contour
    }
}
