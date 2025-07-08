use crate::image_to_coords::request::Request;
use image::GrayImage;

const MOORE_DIRS: [(i32, i32); 8] = [
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
    fn is_black(&self, x: u32, y: u32) -> bool {
        if let Some(pixel) = self.image.get_pixel_checked(x, y) {
            pixel[0] < self.threshold
        } else {
            false
        }
    }

    #[inline(always)]
    fn is_edge(&self, x: u32, y: u32) -> bool {
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
    fn check_pixel(&self, x: u32, y: u32) -> bool {
        match self.edge_detection {
            true => self.is_edge(x, y),
            false => self.is_black(x, y),
        }
    }

    pub fn trace_all_contours_at_height(
        &self,
        at_height: u32,
        visited: &mut [Vec<bool>],
        contours: &mut Vec<Vec<(u32, u32)>>,
    ) {
        for x in 0..self.size {
            if self.check_pixel(x, at_height) && !visited[x as usize][at_height as usize] {
                let mut contour = vec![(x, at_height)];
                visited[x as usize][at_height as usize] = true;
                let mut current = (x, at_height);
                let mut first_loop = true;

                loop {
                    let mut found = false;
                    for (dx, dy) in &MOORE_DIRS {
                        let nx = current.0 as i32 + dx;
                        let ny = current.1 as i32 + dy;

                        if nx >= 0 && ny >= 0 {
                            let (nx, ny) = (nx as u32, ny as u32);
                            if self.check_pixel(nx, ny)
                                && !visited[nx as usize][ny as usize]
                                && ny == at_height
                            {
                                current = (nx, ny);
                                visited[nx as usize][ny as usize] = true;
                                contour.push(current);
                                found = true;

                                for x in 0..self.pix_threshold {
                                    visited[(nx + x) as usize][ny as usize] = true;
                                    visited[nx as usize][(ny + x) as usize] = true;
                                    visited[(nx + x) as usize][(ny + x) as usize] = true;
                                    visited[(nx - x) as usize][ny as usize] = true;
                                    visited[nx as usize][(ny - x) as usize] = true;
                                    visited[(nx - x) as usize][(ny - x) as usize] = true;
                                }
                                break;
                            }
                        }
                    }

                    if !found || (current == contour[0] && !first_loop) {
                        break;
                    }
                    first_loop = false;
                }

                if contour.len() > 1 {
                    contours.push(contour);
                }
            }
        }
    }
    fn is_black_interpolated(&self, x: f32, y: f32) -> bool {
        let ix = x.round() as u32;
        let iy = y.round() as u32;
        if ix >= self.size || iy >= self.size {
            return false;
        }
        self.is_black(ix, iy)
    }

    pub fn black_coord_alg(&self) -> Vec<(u32, u32)> {
        let upscale = 10;
        let mut high_res_coords = Vec::new();

        let width = self.size;
        let height = self.size;

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

    pub fn snake_alg(&self) -> Vec<(u32, u32)> {
        let mut contours = Vec::new();

        let mut x = self.size / 2;
        let mut y = self.size / 2;

        // Directions: Up, Left, Down, Right (CCW spiral)
        let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)];
        let mut dir = 0;

        let mut steps = 1;

        if self.check_pixel(x, y) {
            contours.push((x, y));
        }

        loop {
            for _ in 0..2 {
                // every 2 turns, step size increases
                for _ in 0..steps {
                    let dx = directions[dir].0;
                    let dy = directions[dir].1;

                    // Use i32 to avoid underflow
                    let new_x = x as i32 + dx;
                    let new_y = y as i32 + dy;

                    // Exit if out of bounds
                    if new_x < 0
                        || new_y < 0
                        || new_x >= self.size as i32
                        || new_y >= self.size as i32
                    {
                        return contours;
                    }

                    x = new_x as u32;
                    y = new_y as u32;

                    if self.check_pixel(x, y) {
                        contours.push((x, y));
                    }
                }

                // Turn left (counter-clockwise)
                dir = (dir + 1) % 4;
            }

            // Every two turns, increase step size
            steps += 1;
        }
    }

    /// Find every contour. If the pixel is black and borders white pixels, it gets added to the vec.
    /// We can adjust the level of detail with the pix threshold variable to get the desired result.
    pub fn scanline_alg(&self, index: u32) -> Vec<Vec<(u32, u32)>> {
        let mut visited = vec![vec![false; self.size as usize]; self.size as usize];

        let mut contours = vec![];

        self.trace_all_contours_at_height(index, &mut visited, &mut contours);

        contours
    }

    /// Find every contour. If the pixel is black and borders white pixels, it gets added to the vec.
    /// We can adjust the level of detail with the pix threshold variable to get the desired result.
    pub fn trace_all_contours_alg(&self) -> Vec<Vec<(u32, u32)>> {
        let mut visited = vec![vec![false; self.size as usize]; self.size as usize];

        let mut contours = vec![];

        for y in 0..self.size {
            self.trace_all_contours_at_height(y, &mut visited, &mut contours);
        }

        contours
    }

    /// Simple algorithm that finds only the outlines.
    pub fn trace_outline_alg(&self) -> Vec<(u32, u32)> {
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
