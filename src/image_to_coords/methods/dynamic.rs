use crate::image_to_coords::methods::utils::MOORE_DIRS;
use crate::image_to_coords::request::Request;
use image::GrayImage;
use log::info;

impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    pub fn trace_full_contour(
        &mut self,
        start: (u32, u32),
        visited: &mut [Vec<bool>],
        contour: &mut Vec<(u32, u32)>,
    ) {
        let size = self.size;
        let mut current = start;
        let mut previous_dir = 7;

        contour.push(current);
        let mut first_move = true;

        loop {
            let mut found_next = false;

            for i in 0..8 {
                let dir = (previous_dir + i + 6) % 8;
                let (dx, dy) = MOORE_DIRS[dir];
                let nx = current.0 as i32 + dx;
                let ny = current.1 as i32 + dy;

                if nx >= 0 && ny >= 0 && (nx as u32) < size && (ny as u32) < size {
                    let (ux, uy) = (nx as u32, ny as u32);

                    if self.is_edge(ux, uy) && !visited[ux as usize][uy as usize] {
                        current = (ux, uy);
                        contour.push(current);
                        visited[ux as usize][uy as usize] = true;
                        previous_dir = dir;
                        found_next = true;
                        break;
                    }
                }
            }

            if !found_next {
                break; // Reached end without finding continuation
            }

            if current == start && !first_move {
                break; // Completed loop
            }

            first_move = false;
        }
    }

    pub fn dynamic_contour_parser(&mut self) -> Vec<Vec<(u32, u32)>> {
        let mut visited = vec![vec![false; self.size as usize]; self.size as usize];
        let origin = self.get_starting_point();
        let mut contours = vec![];
        let directions = self.get_dirs();
        // Track position per direction
        let mut positions: Vec<(i32, i32)> = directions
            .iter()
            .map(|(dx, dy)| (origin.0 as i32 + dx, origin.1 as i32 + dy))
            .collect();

        loop {
            let mut all_out_of_bounds = true;

            for (i, (dx, dy)) in directions.iter().enumerate() {
                let (nx, ny) = positions[i];
                info!("parsing: {}{}", nx, ny);

                if nx >= 0 && ny >= 0 && nx < self.size as i32 && ny < self.size as i32 {
                    all_out_of_bounds = false;
                    let ux = nx as u32;
                    let uy = ny as u32;

                    if self.is_edge(ux, uy) && !visited[ux as usize][uy as usize] {
                        let mut contour = vec![];
                        self.trace_full_contour((ux, uy), &mut visited, &mut contour);
                        if contour.len() > 1 {
                            contours.push(contour);
                        }
                    }

                    // Step forward in this direction
                    positions[i] = (nx + dx, ny + dy);
                }
            }

            if all_out_of_bounds {
                break;
            }
        }
        contours
    }
}
