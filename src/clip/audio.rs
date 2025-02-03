use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use crate::time::Ratio;
use hound::{Error, SampleFormat, WavReader};
use itertools::Itertools;
use std::io::Read;
use std::num::FpCategory;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Audio {
    pub sample_rate: u32,
    // On the interval [-1; 1]
    pub samples: Vec<f64>,
}

impl Audio {
    fn duration(&self) -> Duration {
        Duration::from_secs_f64(
            Ratio::new(self.samples.len() as u64, u64::from(self.sample_rate)).to_float(),
        )
    }

    pub fn period(
        &self,
        start: Instant,
        time_signature: &Changing<TimeSignature>,
        tempo: &Changing<Tempo>,
    ) -> Period {
        Period::from_real_time(start, time_signature, tempo, self.duration())
    }
}

// TODO: test
/// Losslessly convert an i32 sample to a f64 sample
fn int_to_float_sample(sample: i32) -> f64 {
    f64::from(sample) / (f64::from(i32::MAX) + 1.0)
}

fn clamp_float_sample(sample: f32) -> f64 {
    let sample = f64::from(sample);
    match sample.classify() {
        FpCategory::Nan => 0.0,
        FpCategory::Infinite => 1_f64.copysign(sample),
        _ => sample.clamp(-1.0, 1.0),
    }
}

impl<R: Read> TryFrom<WavReader<R>> for Audio {
    type Error = Error;

    fn try_from(mut reader: WavReader<R>) -> Result<Self, Self::Error> {
        let spec = reader.spec();
        let samples = match spec.sample_format {
            SampleFormat::Float => reader
                .samples::<f32>()
                .map_ok(clamp_float_sample)
                .try_collect()?,
            SampleFormat::Int => reader
                .samples::<i32>()
                .map_ok(int_to_float_sample)
                .try_collect()?,
        };

        Ok(Audio {
            sample_rate: spec.sample_rate,
            samples,
        })
    }
}
