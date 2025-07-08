use hound;
use std::io::Cursor;

pub struct Request<A: std::ops::Deref<Target = Vec<u8>>> {
    pub audio: A,
    pub result: Vec<(f32, f32)>,
}

/// Every request gets handeled in the Request struct for easy accessibility to variables and to
/// make it easy to refactor or add useability later.
impl<A: std::ops::Deref<Target = Vec<u8>>> Request<A> {
    pub fn process(&mut self) -> Result<(), anyhow::Error> {
        let reader = hound::WavReader::new(Cursor::new(self.audio.clone()))?;
        let spec = reader.spec();
        let num_channels = spec.channels;
        let _sample_rate = spec.sample_rate;

        if num_channels != 2 {
            return Err(anyhow::anyhow!("Only stereo WAV files are supported"));
        }

        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => {
                let max_amplitude = 2f32.powi(spec.bits_per_sample as i32 - 1) - 1.0;
                reader
                    .into_samples::<i32>()
                    .map(|s| s.map(|v| v as f32 / max_amplitude))
                    .collect::<Result<_, _>>()?
            }
            hound::SampleFormat::Float => reader.into_samples::<f32>().collect::<Result<_, _>>()?,
        };

        let mut coords = Vec::with_capacity(samples.len() / 2);
        for chunk in samples.chunks(2) {
            if let [l, r] = chunk {
                coords.push((*l, *r));
            }
        }
        self.result = coords;
        Ok(())
    }
}
