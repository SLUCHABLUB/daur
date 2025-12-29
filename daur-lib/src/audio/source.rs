use crate::Audio;
use crate::audio::InterleavedSamples;
use crate::audio::Sample;
use crate::time;
use rodio::source::SeekError;
use std::time::Duration;

/// An [audio source](rodio::Source) for an [audio](Audio).
#[derive(Clone, Debug)]
#[must_use = "`Source` is an iterator"]
pub struct Source {
    total_duration: Duration,
    samples: InterleavedSamples<'static>,
}

impl Source {
    pub(super) fn new(audio: Audio) -> Source {
        Source {
            total_duration: audio.real_duration().into(),
            samples: audio.into_interleaved_samples(),
        }
    }
}

impl Iterator for Source {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.samples.next().map(Sample::to_f32)
    }
}

impl rodio::Source for Source {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        self.samples.rate().samples_per_second.get()
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }

    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        let duration = time::Duration::from(pos) * self.samples.rate();
        self.samples.skip_forward(duration);

        Ok(())
    }
}
