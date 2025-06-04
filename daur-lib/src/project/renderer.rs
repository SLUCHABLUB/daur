use crate::audio::{Player, sample};
use crate::node::Chain;
use crate::note::event::Sequence;
use crate::sync::Cell;
use crate::time::Instant;
use crate::{Audio, Project};
use executors::Executor as _;
use executors::crossbeam_workstealing_pool::ThreadPool;
use executors::parker::DynParker;
use parking_lot::Mutex;
use saturating_cast::SaturatingCast as _;
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
    unmastered_tracks: Mutex<Vec<Audio<'static>>>,
    audio: OnceLock<Audio<'static>>,
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
    pub(crate) fn restart(&mut self, project: &Project, sample_rate: sample::Rate) {
        let progress = Arc::new(Progress {
            should_stop: Cell::new(false),
            unmastered_tracks: Mutex::new(Vec::with_capacity(project.tracks.len())),
            audio: OnceLock::new(),
        });

        let zero_tracks = project.tracks.is_empty();

        let time_context = project.time_context();

        for track in project.tracks.values() {
            let audio = track.audio_sum(&time_context, sample_rate).into_owned();
            let events = track.events(&time_context, sample_rate);

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
    events: Sequence,
    chain: Chain,
    progress: Arc<Progress>,
    should_play: Arc<Cell<Option<ShouldPlay>>>,
) -> impl FnOnce() {
    move || {
        let sample_rate = input_audio.sample_rate();

        // TODO: un-hardcode
        let batch_size = sample_rate.samples_per_second.get().saturating_cast();
        let batch_duration = sample::Duration {
            samples: batch_size,
        };

        let mut instance = chain.instantiate(sample_rate);

        let mut output_audio = Audio::empty(sample_rate);

        let mut should_process = true;
        let mut position = sample::Instant::START;
        loop {
            let period = sample::Period {
                start: position,
                duration: batch_duration,
            };

            let audio = input_audio.subsection(period);
            let events = events.subsequence(period);

            if !should_process && audio.is_empty() && events.is_empty() {
                break;
            }

            let result = instance.process(batch_duration, &audio, events);

            should_process = result.should_continue;
            output_audio.superpose_with_offset(&result.audio, period.start.since_start);
            position += batch_duration;
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
        audio.superpose(&track);
    }

    // TODO: apply the master plugin chain

    let audio = progress.audio.get_or_init(|| audio);

    if let Some(should_play) = should_play.replace(None) {
        should_play.player.play(audio.clone(), should_play.from);
    }
}
