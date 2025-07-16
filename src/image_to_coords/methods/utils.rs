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
    pub fn get_dirs(&mut self) -> Vec<(i32, i32)> {
        let mut dirs: Vec<(i32, i32)> = Vec::new();
        match &self.directions {
            Some(directions) => {
                for direction in directions {
                    match direction {
                        1 => dirs.push((0, -1)),
                        2 => dirs.push((1, -1)),
                        3 => dirs.push((1, 0)),
                        4 => dirs.push((1, 1)),
                        5 => dirs.push((0, 1)),
                        6 => dirs.push((-1, 1)),
                        7 => dirs.push((-1, 0)),
                        8 => dirs.push((-1, -1)),
                        _ => {}
                    }
                }
                dirs
            }
            None => dirs,
        }
    }

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
        let mut edge_count = 0;
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
                edge_count += 1;
                if edge_count == self.edge_threshold {
                    return true;
                }
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

    pub fn get_starting_point(&self) -> (u32, u32) {
        log::info!("starting point: {:?}", self.starting_point);
        let (x, y) = self.starting_point;
        let nx = (x as f32 / self.canvas_size as f32) * self.size as f32;
        let ny = (y as f32 / self.canvas_size as f32) * self.size as f32;
        log::info!("nx = {}, ny = {}", nx, ny);
        (nx as u32, ny as u32)
    }
}
