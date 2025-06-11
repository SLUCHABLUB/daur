use crate::Audio;
use crate::audio::{Sample, sample};
use bytemuck::cast_slice;
use rubato::{FastFixedIn, PolynomialDegree, Resampler as _};
use std::borrow::Cow;

impl Audio {
    /// Resamples the audio to the given sample rate.
    #[must_use]
    pub fn resample(&self, sample_rate: sample::Rate) -> Cow<Audio> {
        if self.sample_rate == sample_rate {
            return Cow::Borrowed(self);
        }

        match self.try_resample(sample_rate) {
            Ok(audio) => Cow::Owned(audio),
            // This should be unreachable.
            Err(error) => {
                debug_assert!(false, "{error}");
                Cow::Borrowed(self)
            }
        }
    }

    fn try_resample(&self, sample_rate: sample::Rate) -> anyhow::Result<Audio> {
        const ALL_CHANNELS_ENABLED: Option<&[bool]> = None;
        const CHANNEL_COUNT: usize = 2;
        // we want exact resampling
        const MAX_RESAMPLE_RATIO_RELATIVE: f64 = 1.0;

        let input_sample_rate = self.sample_rate.samples_per_second.get();
        let output_sample_rate = sample_rate.samples_per_second.get();

        let ratio = f64::from(output_sample_rate) / f64::from(input_sample_rate);
        let sample_count = self.duration().samples;

        // TODO: allow the user to select an implementation?
        let mut resampler = FastFixedIn::new(
            ratio,
            MAX_RESAMPLE_RATIO_RELATIVE,
            PolynomialDegree::Septic,
            sample_count,
            CHANNEL_COUNT,
        )?;

        let [left, right] = &self.channels;

        let mut output =
            resampler.process(&[cast_slice(left), cast_slice(right)], ALL_CHANNELS_ENABLED)?;

        let left = output
            .pop()
            .unwrap_or_default()
            .into_iter()
            .map(Sample::new)
            .collect();
        let right = output
            .pop()
            .unwrap_or_default()
            .into_iter()
            .map(Sample::new)
            .collect();

        Ok(Audio {
            sample_rate,
            channels: [left, right],
        })
    }
}
