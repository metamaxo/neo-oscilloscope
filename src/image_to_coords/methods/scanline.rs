use crate::image_to_coords::request::Request;

use image::GrayImage;

/// Find every contour. If the pixel is black and borders white pixels, it gets added to the vec.
/// We can adjust the level of detail with the pix threshold variable to get the desired result.
impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    pub fn trace_scanline(&mut self) -> Vec<Vec<(u32, u32)>> {
        match self.scan_type {
            1 => self.outwards_scan(),
            2 => self.inwards_scan(),
            _ => self.linear_scan(),
        }
    }

    pub fn scan(
        &mut self,
        y: u32,
        start_x: u32,
        visited: &mut [Vec<bool>],
    ) -> Option<Vec<(u32, u32)>> {
        match self.scanline_type {
            1 => self.scan_outwards(y, start_x, visited),
            _ => self.scan_inwards(y, start_x, visited),
        }
    }

    pub fn linear_scan(&mut self) -> Vec<Vec<(u32, u32)>> {
        let mut contours = vec![];
        for y in 0..self.size {
            let mut contour = vec![];
            for x in 0..self.size {
                if self.check_pixel(x, y) {
                    contour.push((x, y));
                }
            }
            contours.push(contour);
        }
        contours
    }

    pub fn outwards_scan(&mut self) -> Vec<Vec<(u32, u32)>> {
        let mut visited = vec![vec![false; self.size as usize]; self.size as usize];
        let mut contours = vec![];

        let (start_x, start_y) = self.get_starting_point();
        let directions = self.get_dirs(); // Typically [(0, 1), (0, -1)]

        // Always scan the center line first
        if let Some(center_line) = self.scan(start_y, start_x, &mut visited) {
            if !center_line.is_empty() {
                contours.push(center_line);
            }
        }

        // Expand outward above and below the center
        for offset in 1..self.size {
            let mut stop_count = 0;

            for &(_, dy) in &directions {
                let y = start_y as i32 + offset as i32 * dy;
                if y < 0 || y >= self.size as i32 {
                    stop_count += 1;
                    continue;
                }

                if let Some(contour) = self.scan(y as u32, start_x, &mut visited) {
                    if !contour.is_empty() {
                        contours.push(contour);
                    }
                }
            }

            if stop_count == directions.len() {
                break;
            }
        }

        contours
    }

    pub fn inwards_scan(&mut self) -> Vec<Vec<(u32, u32)>> {
        let mut visited = vec![vec![false; self.size as usize]; self.size as usize];
        let mut contours = vec![];

        let (start_x, start_y) = self.get_starting_point();
        let size = self.size as i32;

        // Always scan the center line first
        if let Some(center_line) = self.scan(start_y, start_x, &mut visited) {
            if !center_line.is_empty() {
                contours.push(center_line);
            }
        }

        let mut top_y = 0i32;
        let mut bottom_y = size - 1;

        while top_y < start_y as i32 || bottom_y > start_y as i32 {
            let mut stop_count = 0;

            // Top inwards
            if top_y < start_y as i32 {
                if let Some(contour) = self.scan(top_y as u32, start_x, &mut visited) {
                    if !contour.is_empty() {
                        contours.push(contour);
                    }
                }
                top_y += 1;
            } else {
                stop_count += 1;
            }

            // Bottom inwards
            if bottom_y > start_y as i32 {
                if let Some(contour) = self.scan(bottom_y as u32, start_x, &mut visited) {
                    if !contour.is_empty() {
                        contours.push(contour);
                    }
                }
                bottom_y -= 1;
            } else {
                stop_count += 1;
            }

            if stop_count == 2 {
                break;
            }
        }

        contours
    }

    pub fn scan_inwards(
        &self,
        at_height: u32,
        start_x: u32,
        visited: &mut [Vec<bool>],
    ) -> Option<Vec<(u32, u32)>> {
        let mut contour = vec![];
        let width = self.size;

        let mut left = 0;
        let mut right = width as i32 - 1;
        let center = start_x as i32;

        loop {
            let mut progressed = false;

            if left as i32 <= center {
                let x = left;
                if self.check_pixel(x, at_height) && !visited[x as usize][at_height as usize] {
                    contour.push((x, at_height));
                    visited[x as usize][at_height as usize] = true;
                }
                left += 1;
                progressed = true;
            }

            if right >= center {
                let x = right as u32;
                if self.check_pixel(x, at_height) && !visited[x as usize][at_height as usize] {
                    contour.push((x, at_height));
                    visited[x as usize][at_height as usize] = true;
                }
                right -= 1;
                progressed = true;
            }

            if !progressed || (left as i32 > center && right < center) {
                break;
            }
        }

        if contour.is_empty() {
            None
        } else {
            Some(contour)
        }
    }

    pub fn scan_outwards(
        &self,
        at_height: u32,
        start_x: u32,
        visited: &mut [Vec<bool>],
    ) -> Option<Vec<(u32, u32)>> {
        let mut contour = vec![];
        let width = self.size;

        let mut left = start_x as i32 - 1;
        let mut right = start_x as i32 + 1;

        if self.check_pixel(start_x, at_height) && !visited[start_x as usize][at_height as usize] {
            contour.push((start_x, at_height));
            visited[start_x as usize][at_height as usize] = true;
        }

        loop {
            let mut progressed = false;

            if left >= 0 {
                let x = left as u32;
                if self.check_pixel(x, at_height) && !visited[x as usize][at_height as usize] {
                    contour.push((x, at_height));
                    visited[x as usize][at_height as usize] = true;
                }
                left -= 1;
                progressed = true;
            }

            if right < width as i32 {
                let x = right as u32;
                if self.check_pixel(x, at_height) && !visited[x as usize][at_height as usize] {
                    contour.push((x, at_height));
                    visited[x as usize][at_height as usize] = true;
                }
                right += 1;
                progressed = true;
            }

            if !progressed {
                break;
            }
        }

        if contour.is_empty() {
            None
        } else {
            Some(contour)
        }
    }
}
