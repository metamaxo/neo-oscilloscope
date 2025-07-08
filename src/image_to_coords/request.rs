use crate::image_to_coords::mode::Mode;
use image::GrayImage;

pub struct Request<Im: std::ops::Deref<Target = GrayImage>> {
    pub mode: Mode,
    pub image: Im,
    pub threshold: u8,
    pub pix_threshold: u32,
    pub result: Vec<(f32, f32)>,
    pub interpolate: bool,
    pub int_amount: usize,
    pub size: u32,
    pub edge_detection: bool,
}

/// Every request gets handeled in the Request struct for easy accessibility to variables and to
/// make it easy to refactor or add useability later.
impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    pub fn process(&mut self) {
        match self.mode {
            Mode::Outline => Self::outline(self),
            Mode::Full => Self::full_contour(self),
            Mode::Scan => Self::scan_lines(self),
            Mode::Snake => Self::snake(self),
            Mode::Black => Self::black(self),
        }
    }

    pub fn black(&mut self) {
        let black_coords = self.black_coord_alg();
        self.result = self.normalize(&black_coords);
    }

    pub fn snake(&mut self) {
        let outline = self.snake_alg();
        self.result = self.normalize(&outline);
    }

    // Get coords for outline mode
    pub fn outline(&mut self) {
        let outline = self.trace_outline_alg();
        let normalized = self.normalize(&outline);
        self.result = self.interpolate(&normalized)
    }

    // Get coords for full mode, normalize and interpolate are chained together using 2 map
    // functions for easy readabilty and customization.
    pub fn full_contour(&mut self) {
        self.result = self
            .trace_all_contours_alg()
            .iter()
            .flat_map(|outline| {
                let normalized = self.normalize(outline);
                self.interpolate(&normalized)
            })
            .collect();
    }

    pub fn scan_lines(&mut self) {
        let scanlines: Vec<(u32, u32)> = (0..self.size)
            .flat_map(|index| self.scanline_alg(index))
            .flatten() // Flatten the Vec<Vec<_>> into Vec<_>
            .collect();

        self.result = self.interpolate(&self.normalize(&scanlines));
    }
}
