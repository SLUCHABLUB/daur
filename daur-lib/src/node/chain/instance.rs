use crate::audio::sample::Duration;
use crate::audio::{Sample, sample};
use crate::node::ProcessResult;
use crate::note::event::Subsequence;
use crate::note::{Event, Pitch};
use crate::{Audio, Id, Note};
use std::collections::HashMap;

pub(crate) struct Instance {
    sample_rate: sample::Rate,
    position: sample::Instant,

    keys: HashMap<Id<Note>, Pitch>,
}

impl Instance {
    pub(super) fn new(sample_rate: sample::Rate) -> Instance {
        Instance {
            sample_rate,
            position: sample::Instant::START,
            keys: HashMap::new(),
        }
    }

    pub(crate) fn process(
        &mut self,
        duration: Duration,
        input_audio: &Audio,
        events: Subsequence,
    ) -> ProcessResult {
        // TODO: pass to a plugin instance

        let mut output_audio = Audio::with_capacity(input_audio.sample_rate(), duration);

        let buffer_size = duration.samples;

        for index in 0..buffer_size {
            let instant = sample::Instant::from_index(index);

            for event in events.get(instant) {
                match event {
                    Event::NoteOn { id, pitch } => {
                        self.keys.insert(*id, *pitch);
                    }
                    Event::NoteOff(id) => {
                        self.keys.remove(id);
                    }
                }
            }

            let [left_input, right_input] = input_audio.sample_pair(instant);
            let [left_output, right_output] = output_audio.sample_pair_mut(instant);

            let mut delta = Sample::ZERO;

            #[expect(clippy::iter_over_hash_type, reason = "order is irrelevant")]
            for pitch in self.keys.values() {
                let frequency = pitch.frequency() / self.sample_rate.hz();
                #[expect(clippy::cast_precision_loss, reason = "approximating is fine")]
                let time = self.position.since_start.samples as f32;

                delta += Sample::new(f32::sin(frequency * time));
            }

            *left_output = left_input + delta;
            *right_output = right_input + delta;
        }

        self.position += input_audio.duration();

        ProcessResult {
            audio: output_audio,
            should_continue: false,
        }
    }
}
