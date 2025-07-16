use crate::image_to_coords::method::Method;
use crate::image_to_coords::processing_utils;
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
    pub canvas_size: u32,
    pub starting_point: (f64, f64),
    pub directions: Option<Vec<u32>>,
    pub scan_type: u32,
    pub scanline_type: u32,
    pub snake_step_amount: u32,
    pub horizontal: bool,
    pub scramble: bool,
    pub double_trace: bool,
    pub edge_threshold: u8,
    pub flatten: bool,
}

/// Every request gets handeled in the Request struct for easy accessibility to variables and to
/// make it easy to refactor or add useability later.
impl<Im: std::ops::Deref<Target = GrayImage>> Request<Im> {
    // Method dispatcher
    pub fn process(&mut self) {
        match self.method {
            Method::Outline => Self::outline(self),
            Method::Full => Self::full_contour(self),
            Method::Scan => Self::scanline(self),
            Method::Snake => Self::snake(self),
            Method::Black => Self::black(self),
            Method::Dynamic => Self::dynamic(self),
            Method::Zigzag => Self::zigzag(self),
        }
    }
    // Processing helpers.

    // Process results that were collected into a single array of tuples.
    pub fn process_result(&mut self, outline: &[(u32, u32)]) {
        let normalized = self.normalize(outline);
        let scrambled = self.scrambler(normalized);
        self.result = self.interpolate(&scrambled);
    }

    // Process results that were collected into nested arrays where each array has to be processed
    // individually.
    pub fn process_result_vec(&mut self, outlines: &[&[(u32, u32)]]) {
        self.result = outlines
            .iter()
            .flat_map(|outline| {
                let normalized = self.normalize(outline);
                let scrambled = self.scrambler(normalized);
                self.interpolate(&scrambled)
            })
            .collect();
    }

    // Checks if nested arrays need to be flattened before or after processing.
    pub fn check_flatten(&mut self, outlines: &[&[(u32, u32)]]) {
        if self.flatten {
            let flat: Vec<(u32, u32)> = outlines.iter().flat_map(|o| *o).copied().collect();
            self.process_result(&flat);
        } else {
            self.process_result_vec(outlines);
        }
    }

    // Non oscilloscope methods

    // Black method is used to find all the black pixels within the allowed threshold. We then use
    // these pixels to create a mask by not allowing pixels that aren't in the black pixel set to
    // be drawn. This method is never used for creating visuals.
    pub fn black(&mut self) {
        self.process_result(&self.trace_black_coords());
    }

    // Classic oscilloscope methods

    // Outline only finds the outer most outline, Unlike the other methods, this method would work
    // for creating oscilloscope art that works on a classic oscilloscope.
    pub fn outline(&mut self) {
        let result = self.trace_outline();
        self.process_result(&result);
    }

    // Full contour finds every outline.
    pub fn full_contour(&mut self) {
        let result = self.trace_all_outlines();
        self.check_flatten(&self.to_slice_refs(&result));
    }

    // Neo oscilloscope methods

    // Snake method starts at the center and spirals outwards, creating a spiraling effect when
    // playback rate is increased.
    pub fn snake(&mut self) {
        let result = self.trace_snake();
        self.process_result(&result);
    }

    // Zigzag method iterates in 2 directions, when it can't proceed forwards, it jumps to the next
    // row and moves backwards.
    pub fn zigzag(&mut self) {
        let result = self.trace_zig_zag();
        self.process_result(&result);
    }

    // Scan line method finds every target on the current height only.
    pub fn scanline(&mut self) {
        let result = self.trace_scanline();
        self.check_flatten(&self.to_slice_refs(&result));
    }

    pub fn dynamic(&mut self) {
        let result = self.dynamic_contour_parser();
        self.check_flatten(&self.to_slice_refs(&result));
    }
}
