use hound;
use log::info;
use std::io::Cursor;

pub struct Request<C: std::ops::Deref<Target = Vec<(f32, f32)>>> {
    pub coords: C,
    pub sample_rate: u32,
    pub playback_rate: f32,
    pub result: Vec<u8>,
}

/// Every request gets handeled in the Request struct for easy accessibility to variables and to
/// make it easy to refactor or add useability later.
impl<C: std::ops::Deref<Target = Vec<(f32, f32)>>> Request<C> {
    pub fn process(&mut self) {
        let samples = self.coords.to_vec(); // no resampling here!
        info!("coords len: {}", self.coords.len());

        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut buffer = Cursor::new(Vec::new());
        let mut writer = hound::WavWriter::new(&mut buffer, spec).unwrap();

        for (x, y) in samples {
            writer
                .write_sample((x.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                .unwrap();
            writer
                .write_sample((y.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                .unwrap();
        }

        writer.finalize().unwrap();

        self.result = buffer.into_inner();
    }
}
