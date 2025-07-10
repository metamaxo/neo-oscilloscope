use crate::image_to_coords::method::Method;
use image::GrayImage;

pub struct Request<Im: std::ops::Deref<Target = GrayImage>> {
    pub method: Method,
    pub image: Im,
    pub threshold: u8,
    pub pix_threshold: u32,
    pub spread_type: u32,
    pub result: Vec<(f32, f32)>,
    pub interpolate: bool,
    pub int_amount: usize,
    pub size: u32,
    pub edge_detection: bool,
}

/// Every request gets handeled in the Request struct for easy accessibility to variables and to
/// make it easy to refactor or add useability later.
impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    // Method dispatcher
    pub fn process(&mut self) {
        match self.method {
            Method::Outline => Self::outline(self),
            Method::Full => Self::full_contour(self),
            Method::Scan => Self::scan_lines(self),
            Method::Snake => Self::snake(self),
            Method::Black => Self::black(self),
        }
    }

    // Classic oscilloscope methods

    // Outline only finds the outer most outline, Unlike the other methods, this method would work
    // for creating oscilloscope art that works on a classic oscilloscope.
    pub fn outline(&mut self) {
        let outline = self.trace_outline();
        let normalized = self.normalize(&outline);
        self.result = self.interpolate(&normalized)
    }

    // Full contour finds every outline.
    pub fn full_contour(&mut self) {
        self.result = self
            .trace_all_outlines()
            .iter()
            .flat_map(|outline| {
                let normalized = self.normalize(outline);
                self.interpolate(&normalized)
            })
            .collect();
    }

    // Neo oscilloscope methods

    // Black method is used to find all the black pixels within the allowed threshold. We then use
    // these pixels to create a mask by not allowing pixels that aren't in the black pixel set to
    // be drawn.
    pub fn black(&mut self) {
        let black_coords = self.trace_black_coords();
        self.result = self.normalize(&black_coords);
    }

    // Snake method starts at the center and spirals outwards, creating a spiraling effect when
    // playback rate is increased.
    pub fn snake(&mut self) {
        let outline = self.trace_snake();
        self.result = self.normalize(&outline);
    }

    // Scan line method finds every edge on the current height only.
    pub fn scan_lines(&mut self) {
        let scanlines: Vec<(u32, u32)> = (0..self.size)
            .flat_map(|index| self.trace_scanline(index))
            .flatten() // Flatten the Vec<Vec<_>> into Vec<_>
            .collect();

        self.result = self.interpolate(&self.normalize(&scanlines));
    }
}
