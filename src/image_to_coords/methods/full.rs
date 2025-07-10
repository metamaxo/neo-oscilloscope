use crate::image_to_coords::methods::utils::MOORE_DIRS;
use crate::image_to_coords::request::Request;
use image::GrayImage;
use log::info;

impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    pub fn trace_full_contour_from(
        &self,
        start: (u32, u32),
        visited: &mut [Vec<bool>],
        contour: &mut Vec<(u32, u32)>,
    ) {
        let mut stack = vec![start];
        visited[start.0 as usize][start.1 as usize] = true;

        while let Some((x, y)) = stack.pop() {
            contour.push((x, y));

            for (dx, dy) in &MOORE_DIRS {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 && nx < self.size as i32 && ny < self.size as i32 {
                    let (nx, ny) = (nx as u32, ny as u32);
                    if self.check_pixel(nx, ny) && !visited[nx as usize][ny as usize] {
                        visited[nx as usize][ny as usize] = true;
                        stack.push((nx, ny));
                    }
                }
            }
        }
    }

    /// Find every contour. If the pixel is black and borders white pixels, it gets added to the vec.
    /// We can adjust the level of detail with the pix threshold variable to get the desired result.
    pub fn trace_all_outlines(&self) -> Vec<Vec<(u32, u32)>> {
        let mut visited = vec![vec![false; self.size as usize]; self.size as usize];
        let mut contours = vec![];

        for y in 0..self.size {
            for x in 0..self.size {
                if self.check_pixel(x, y) && !visited[x as usize][y as usize] {
                    let mut contour = vec![];
                    self.trace_full_contour_from((x, y), &mut visited, &mut contour);
                    if contour.len() > 1 {
                        contours.push(contour);
                    }
                }
            }
        }

        contours
    }
}
