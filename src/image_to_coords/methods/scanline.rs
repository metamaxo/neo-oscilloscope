use crate::image_to_coords::methods::utils::MOORE_DIRS;
use crate::image_to_coords::request::Request;

use image::GrayImage;

/// Find every contour. If the pixel is black and borders white pixels, it gets added to the vec.
/// We can adjust the level of detail with the pix threshold variable to get the desired result.
impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    pub fn trace_scanline(&self, index: u32) -> Vec<Vec<(u32, u32)>> {
        let mut visited = vec![vec![false; self.size as usize]; self.size as usize];

        let mut contours = vec![];

        self.trace_all_contours_at_height(index, &mut visited, &mut contours);

        contours
    }
    pub fn trace_all_contours_at_height(
        &self,
        at_height: u32,
        visited: &mut [Vec<bool>],
        contours: &mut Vec<Vec<(u32, u32)>>,
    ) {
        let max_size = self.size as i32;
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

                        if nx >= 0 && ny >= 0 && nx <= max_size && ny <= max_size {
                            let (nx, ny) = (nx as u32, ny as u32);
                            if self.check_pixel(nx, ny)
                                && !visited[nx as usize][ny as usize]
                                && ny == at_height
                            {
                                current = (nx, ny);
                                visited[nx as usize][ny as usize] = true;
                                contour.push(current);
                                found = true;

                                self.check_threshold(nx, ny, visited);

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
}
