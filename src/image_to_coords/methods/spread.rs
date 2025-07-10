use crate::image_to_coords::request::Request;
use image::GrayImage;

impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    pub fn check_threshold(&self, nx: u32, ny: u32, visited: &mut [Vec<bool>]) {
        match self.spread_type {
            1 => self.check_square_spread(nx, ny, visited),
            2 => self.check_round_spread(nx, ny, visited),
            3 => self.check_star_spread(nx, ny, visited),
            4 => self.check_simple_spread(nx, ny, visited),
            5 => self.check_horizontal_spread(nx, ny, visited),
            6 => self.check_vertical_spread(nx, ny, visited),
            _ => {}
        }
    }

    fn check_horizontal_spread(&self, nx: u32, ny: u32, visited: &mut [Vec<bool>]) {
        let max = self.size as usize;
        let cx = nx as i32;
        let cy = ny as i32;
        let r = self.pix_threshold as i32;

        for i in 1..=r {
            let offsets = [(cx + i, cy), (cx - i, cy)];

            for (x, y) in offsets {
                if x >= 0 && y >= 0 && (x as usize) < max && (y as usize) < max {
                    visited[x as usize][y as usize] = true;
                }
            }
        }
    }

    fn check_vertical_spread(&self, nx: u32, ny: u32, visited: &mut [Vec<bool>]) {
        let max = self.size as usize;
        let cx = nx as i32;
        let cy = ny as i32;
        let r = self.pix_threshold as i32;

        for i in 1..=r {
            let offsets = [(cx, cy + i), (cx, cy - i)];

            for (x, y) in offsets {
                if x >= 0 && y >= 0 && (x as usize) < max && (y as usize) < max {
                    visited[x as usize][y as usize] = true;
                }
            }
        }
    }

    fn check_simple_spread(&self, nx: u32, ny: u32, visited: &mut [Vec<bool>]) {
        let max = self.size as usize;
        let cx = nx as i32;
        let cy = ny as i32;
        let r = self.pix_threshold as i32;

        for i in 1..=r {
            let offsets = [(cx + i, cy), (cx - i, cy), (cx, cy + i), (cx, cy - i)];

            for (x, y) in offsets {
                if x >= 0 && y >= 0 && (x as usize) < max && (y as usize) < max {
                    visited[x as usize][y as usize] = true;
                }
            }
        }
    }

    fn check_star_spread(&self, nx: u32, ny: u32, visited: &mut [Vec<bool>]) {
        let max = self.size as usize;
        let cx = nx as i32;
        let cy = ny as i32;
        let r = self.pix_threshold as i32;

        for i in 1..=r {
            let offsets = [
                (cx + i, cy),
                (cx - i, cy),
                (cx, cy + i),
                (cx, cy - i),
                (cx + i, cy + i),
                (cx - i, cy + i),
                (cx + i, cy - i),
                (cx - i, cy - i),
            ];

            for (x, y) in offsets {
                if x >= 0 && y >= 0 && (x as usize) < max && (y as usize) < max {
                    visited[x as usize][y as usize] = true;
                }
            }
        }
    }

    fn check_round_spread(&self, nx: u32, ny: u32, visited: &mut [Vec<bool>]) {
        let max = self.size as usize;
        let r = self.pix_threshold as i32;
        let cx = nx as i32;
        let cy = ny as i32;
        let r2 = r * r;

        for dy in -r..=r {
            let y = cy + dy;
            if y < 0 || y as usize >= max {
                continue;
            }

            for dx in -r..=r {
                let x = cx + dx;
                if x < 0 || x as usize >= max {
                    continue;
                }

                if dx * dx + dy * dy <= r2 {
                    visited[x as usize][y as usize] = true;
                }
            }
        }
    }

    fn check_square_spread(&self, nx: u32, ny: u32, visited: &mut [Vec<bool>]) {
        let max = self.size as usize;
        let r = self.pix_threshold as i32;
        let cx = nx as i32;
        let cy = ny as i32;

        for dy in -r..=r {
            let y = cy + dy;
            if y < 0 || y as usize >= max {
                continue;
            }

            for dx in -r..=r {
                let x = cx + dx;
                if x < 0 || x as usize >= max {
                    continue;
                }

                visited[x as usize][y as usize] = true;
            }
        }
    }
}
