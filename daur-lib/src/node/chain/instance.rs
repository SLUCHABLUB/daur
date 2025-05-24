use crate::audio::SampleRate;
use anyhow::Result;
use clack_host::events::spaces::CoreEventSpace;
use clack_host::prelude::{InputAudioBuffers, InputEvents, OutputAudioBuffers, ProcessStatus};
use clack_plugin::prelude::ChannelPair;
use itertools::Itertools as _;
use saturating_cast::SaturatingCast as _;
use std::collections::{HashMap, HashSet};
use std::iter::zip;

pub(crate) struct Instance {
    sample_rate: SampleRate,
    sample: usize,

    // TODO: remove this abomination
    keys: HashMap<usize, HashSet<u16>>,
}

impl Instance {
    pub(super) fn new(sample_rate: SampleRate) -> Self {
        Instance {
            sample_rate,
            sample: 0,
            keys: HashMap::new(),
        }
    }

    pub(crate) fn process<'buffers>(
        &mut self,
        audio_inputs: &'buffers InputAudioBuffers<'buffers>,
        audio_outputs: &'buffers mut OutputAudioBuffers<'buffers>,
        events: &InputEvents,
    ) -> Result<ProcessStatus> {
        // TODO: pass to a plugin instance

        #[expect(
            clippy::cast_precision_loss,
            reason = "part of the temporary implementation"
        )]
        let a440_frequency = 440.0 / self.sample_rate.samples_per_second.get() as f32;

        let mut audio = audio_outputs.as_plugin_audio_with_inputs(audio_inputs);

        let Some(mut main_port) = audio.port_pair(0) else {
            return Ok(ProcessStatus::Sleep);
        };

        let Some(channels) = main_port.channels()?.into_f32() else {
            return Ok(ProcessStatus::Sleep);
        };

        for (channel_index, mut channel_pair) in channels.into_iter().enumerate() {
            match &mut channel_pair {
                ChannelPair::InputOnly(_) => continue,
                ChannelPair::OutputOnly(buf) => buf.fill(0.0),
                ChannelPair::InputOutput(input, output) => {
                    for (input, output) in zip(*input, output.iter_mut()) {
                        *output = *input;
                    }
                }
                ChannelPair::InPlace(_) => {}
            }

            let Some(output) = channel_pair.output_mut() else {
                return Ok(ProcessStatus::Sleep);
            };

            let mut events = events.iter();
            let keys = self.keys.entry(channel_index).or_default();

            // Process the events.
            for (index, sample) in output.iter_mut().enumerate() {
                let index = self.sample.saturating_add(index);

                let events = events.take_while_ref(|event| {
                    event.header().time() <= index.saturating_cast::<u32>()
                });

                for event in events {
                    let Some(event) = event.as_core_event() else {
                        continue;
                    };

                    #[expect(
                        clippy::wildcard_enum_match_arm,
                        reason = "this is a temporary implementation"
                    )]
                    match event {
                        CoreEventSpace::NoteOn(event) => {
                            let Some(key) = event.pckn().key.into_specific() else {
                                continue;
                            };
                            keys.insert(key);
                        }
                        CoreEventSpace::NoteOff(event) => {
                            let Some(key) = event.pckn().key.into_specific() else {
                                continue;
                            };
                            keys.remove(&key);
                        }
                        _ => (),
                    }
                }

                #[expect(
                    clippy::cast_precision_loss,
                    reason = "this is a temporary implementation"
                )]
                let time = index as f32;

                #[expect(clippy::iter_over_hash_type, reason = "addition is commutative")]
                for key in &*keys {
                    let key_offset = f32::from(*key) - 69.0;

                    let frequency = a440_frequency * f32::powf(2.0, key_offset / 12.0);

                    *sample += f32::sin(time * frequency);
                }
            }
        }

        self.sample = self
            .sample
            .saturating_add(audio_inputs.frames_count().unwrap_or(0).saturating_cast());

        Ok(ProcessStatus::Sleep)
    }
}
