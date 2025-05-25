use crate::audio::{Player, Sample, sample};
use crate::node::Chain;
use crate::note::Event;
use crate::sync::Cell;
use crate::time::Instant;
use crate::{Audio, Project, project};
use clack_host::events::UnknownEvent;
use clack_host::prelude::{
    AudioPortBuffer, AudioPortBufferType, AudioPorts, InputChannel, InputEvents,
};
use executors::Executor as _;
use executors::crossbeam_workstealing_pool::ThreadPool;
use executors::parker::DynParker;
use itertools::Itertools as _;
use parking_lot::Mutex;
use saturating_cast::SaturatingCast as _;
use sorted_vec::SortedVec;
use std::array::from_fn;
use std::iter::zip;
use std::mem::{replace, take};
use std::sync::{Arc, OnceLock};

#[derive(Default)]
pub(crate) struct Renderer {
    /// The thread pool.
    ///
    /// This executor type is used due to this recommendation in the `executors` crate's readme:
    ///
    /// > If you don't know what hardware your code is going to run on, use the crossbeam_workstealing_pool
    thread_pool: ThreadPool<DynParker>,
    // data that is shared across threads and should be reset on rerender
    progress: Arc<Progress>,

    // this is not part of progress since as it should it not reset
    /// This is set if the audio should play.
    should_play: Arc<Cell<Option<ShouldPlay>>>,
}

#[derive(Default)]
struct Progress {
    should_stop: Cell<bool>,
    unmastered_tracks: Mutex<Vec<Audio>>,
    audio: OnceLock<Audio>,
}

struct ShouldPlay {
    from: Instant,
    player: Player,
}

impl Renderer {
    pub(crate) fn play_when_finished(&mut self, from: Instant, player: Player) {
        let main_player = player.clone();

        // it is important that we set this before checking the audio cell
        // whilst it may run `Player::play` twice, it will guarantee that it is run
        self.should_play.set(Some(ShouldPlay { from, player }));

        if let Some(audio) = self.progress.audio.get() {
            // check that the mastering thread has not started playing the audio
            if self.should_play.replace(None).is_none() {
                return;
            }

            main_player.play(audio.clone(), from);
        }
    }

    // TODO: the audio up to the point of the change may be reused
    pub(crate) fn restart(
        &mut self,
        project: &Project,
        project_settings: &project::Settings,
        sample_rate: sample::Rate,
    ) {
        let progress = Arc::new(Progress {
            should_stop: Cell::new(false),
            unmastered_tracks: Mutex::new(Vec::with_capacity(project.tracks.len())),
            audio: OnceLock::new(),
        });

        let zero_tracks = project.tracks.is_empty();

        for track in project.tracks.values() {
            let audio = track.audio_sum(project_settings, sample_rate);
            let events = track.events(project_settings, sample_rate);
            // TODO: take from the track
            let chain = Chain::default();

            let progress = Arc::clone(&progress);
            let should_play = Arc::clone(&self.should_play);

            self.thread_pool
                .execute(rendering_job(audio, events, chain, progress, should_play));
        }

        if zero_tracks {
            master(Vec::new(), sample_rate, &progress, &self.should_play);
        }

        let old_progress = replace(&mut self.progress, progress);

        // Stop the threads that are rendering the old project
        old_progress.should_stop.set(true);
    }
}

fn rendering_job(
    input_audio: Audio,
    events: SortedVec<Event>,
    chain: Chain,
    progress: Arc<Progress>,
    should_play: Arc<Cell<Option<ShouldPlay>>>,
) -> impl FnOnce() {
    move || {
        let sample_rate = input_audio.sample_rate;

        let mut input_ports = AudioPorts::with_capacity(2, 1);
        let mut output_ports = AudioPorts::with_capacity(2, 1);

        // TODO: un-hardcode
        let batch_size = sample_rate.samples_per_second.get().saturating_cast();
        let batch_duration = sample::Duration {
            samples: batch_size,
        };

        let mut instance = chain.instantiate(sample_rate);

        let mut events = events.iter().map(Event::as_ref);

        let mut output_audio = Audio::empty(sample_rate);

        for (batch_index, audio_batch) in input_audio.samples.chunks(batch_size).enumerate() {
            // TODO: use 64-bit if the plugin supports it
            let mut input_buffers: [Vec<f32>; 2] = from_fn(|_| vec![0.0; batch_size]);
            let mut output_buffers: [Vec<f32>; 2] = from_fn(|_| vec![0.0; batch_size]);

            for (index, pair) in audio_batch.iter().enumerate() {
                if let Some(buffer) = input_buffers[0].get_mut(index) {
                    *buffer = pair.left.to_f32();
                }
                if let Some(buffer) = input_buffers[1].get_mut(index) {
                    *buffer = pair.right.to_f32();
                }
            }

            let audio_input = input_ports.with_input_buffers([AudioPortBuffer {
                latency: 0,
                channels: AudioPortBufferType::f32_input_only(
                    input_buffers.iter_mut().map(InputChannel::constant),
                ),
            }]);

            let mut audio_output = output_ports.with_output_buffers([AudioPortBuffer {
                latency: 0,
                channels: AudioPortBufferType::f32_output_only(
                    output_buffers.iter_mut().map(Vec::as_mut_slice),
                ),
            }]);

            let this_batch_duration = sample::Duration {
                samples: audio_batch.len(),
            };

            let next_batch_start = sample::Instant {
                since_start: batch_duration * batch_index + this_batch_duration,
            };

            let events: Vec<_> = events
                .take_while_ref(|event| start_of(event) < next_batch_start)
                .collect();

            let events = InputEvents::from_buffer(&events);

            // TODO: check the return value to see if we should continue
            // TODO: open a popup somehow
            let _result = instance.process(&audio_input, &mut audio_output, &events);

            for (left, right) in zip(&output_buffers[0], &output_buffers[1]) {
                output_audio.samples.push(sample::Pair {
                    left: Sample::from_f32(*left),
                    right: Sample::from_f32(*right),
                });
            }

            if progress.should_stop.get() {
                return;
            }
        }

        let mut tracks = progress.unmastered_tracks.lock();
        tracks.push(output_audio);
        if tracks.len() == tracks.capacity() {
            let tracks = take(&mut *tracks);

            master(tracks, sample_rate, &progress, &should_play);
        }
    }
}

fn master(
    tracks: Vec<Audio>,
    sample_rate: sample::Rate,
    progress: &Progress,
    should_play: &Cell<Option<ShouldPlay>>,
) {
    let mut audio = Audio::empty(sample_rate);

    for track in tracks {
        audio += &track;
    }

    // TODO: apply the master plugin chain

    let audio = progress.audio.get_or_init(|| audio);

    if let Some(should_play) = should_play.replace(None) {
        should_play.player.play(audio.clone(), should_play.from);
    }
}

// TODO: move (maybe to an extension trait)
fn start_of(event: &UnknownEvent) -> sample::Instant {
    sample::Instant {
        since_start: sample::Duration {
            samples: event.header().time().saturating_cast(),
        },
    }
}
