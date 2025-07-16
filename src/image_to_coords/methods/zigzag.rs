use crate::image_to_coords::request::Request;
use image::GrayImage;

impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    // We move along the x or y axis, when the end is reached, we go backwards on the next row
    pub fn trace_zig_zag(&mut self) -> Vec<(u32, u32)> {
        // Creates x and y for forwards and backwards, and for vertical and horizontal
        fn get_x_y(
            row: u32,
            index: u32,
            forwards: bool,
            horizontal: bool,
            size: u32,
        ) -> (u32, u32) {
            if horizontal {
                if forwards {
                    (index, row)
                } else {
                    (size - index, row)
                }
            } else {
                if forwards {
                    (row, index)
                } else {
                    (row, size - index)
                }
            }
        }

        let mut contours = Vec::new();

        let mut forwards = true;

        for row in 0..self.size {
            for index in 0..self.size {
                let (x, y) = get_x_y(row, index, forwards, self.horizontal, self.size);
                if self.check_pixel(x, y) {
                    contours.push((x, y))
                }
            }

            if forwards == true {
                forwards = false
            } else {
                forwards = true
            }
        }

        contours
    }
}
