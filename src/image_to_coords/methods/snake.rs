use crate::image_to_coords::request::Request;

use image::GrayImage;

impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    pub fn trace_snake(&self) -> Vec<(u32, u32)> {
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
}
