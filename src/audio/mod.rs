pub mod source;

use crate::audio::source::AudioSource;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use hound::{Error, SampleFormat, WavReader};
use itertools::{EitherOrBoth, Itertools};
use std::io::Read;
use std::num::FpCategory;
use std::time::Duration;

#[derive(Clone, PartialEq, Debug)]
pub struct Audio {
    sample_rate: u32,
    // On the interval [-1; 1]
    // TODO: use non nan type and derive Eq
    channels: [Vec<f64>; 2],
}

impl Audio {
    pub fn sample_count(&self) -> usize {
        usize::max(self.channels[0].len(), self.channels[1].len())
    }

    pub fn duration(&self) -> Duration {
        const NANOS_PER_SEC: u32 = 1_000_000_000;

        let sample_count = self.sample_count() as u64;
        let sample_rate = u64::from(self.sample_rate);

        let seconds = sample_count / sample_rate;
        #[allow(clippy::cast_possible_truncation)]
        let remainder = (sample_count % sample_rate) as u32;

        let nanos = (remainder * NANOS_PER_SEC) / self.sample_rate;

        Duration::new(seconds, nanos)
    }

    pub fn mono_samples(&self) -> impl Iterator<Item = f64> + use<'_> {
        Itertools::zip_longest(self.channels[0].iter(), self.channels[1].iter()).map(|either| {
            match either {
                EitherOrBoth::Both(l, r) => (l + r) / 2.0,
                EitherOrBoth::Left(sample) | EitherOrBoth::Right(sample) => *sample,
            }
        })
    }

    pub fn period(
        &self,
        start: Instant,
        time_signature: &Changing<TimeSignature>,
        tempo: &Changing<Tempo>,
    ) -> Period {
        Period::from_real_time(start, time_signature, tempo, self.duration())
    }

    pub fn to_source(&self, offset: usize) -> AudioSource {
        AudioSource::new(self.clone(), offset)
    }
}

impl Eq for Audio {}

// TODO: test
/// Losslessly convert an i32 sample to a f64 sample
fn int_to_float_sample(sample: i32) -> f64 {
    f64::from(sample) / (f64::from(i32::MAX) + 1.0)
}

fn clamp_float_sample(sample: f32) -> f64 {
    let sample = f64::from(sample);
    match sample.classify() {
        FpCategory::Nan => 0.0,
        _ => sample.clamp(-1.0, 1.0),
    }
}

impl<R: Read> TryFrom<WavReader<R>> for Audio {
    type Error = Error;

    fn try_from(mut reader: WavReader<R>) -> Result<Self, Self::Error> {
        let spec = reader.spec();
        let samples: Vec<f64> = match spec.sample_format {
            SampleFormat::Float => reader
                .samples::<f32>()
                .map_ok(clamp_float_sample)
                .try_collect()?,
            SampleFormat::Int => reader
                .samples::<i32>()
                .map_ok(int_to_float_sample)
                .try_collect()?,
        };

        let channels = match spec.channels {
            1 => [samples.clone(), samples],
            2 => samples
                .chunks_exact(2)
                .map(|chunk| (chunk[0], chunk[1]))
                .unzip()
                .into(),
            _ => return Err(Error::Unsupported),
        };

        Ok(Audio {
            sample_rate: spec.sample_rate,
            channels,
        })
    }
}
