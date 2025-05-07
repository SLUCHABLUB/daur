use crate::audio::{Player, SampleRate};
use crate::time::Mapping;
use crate::time::real::Instant;
use crate::track::RenderStream;
use crate::{Audio, Cell, Track};
use executors::Executor as _;
use executors::crossbeam_workstealing_pool::ThreadPool;
use executors::parker::DynParker;
use parking_lot::Mutex;
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
        tracks: &[Arc<Track>],
        mapping: &Mapping,
        sample_rate: SampleRate,
    ) {
        let progress = Arc::new(Progress {
            should_stop: Cell::new(false),
            unmastered_tracks: Mutex::new(Vec::with_capacity(tracks.len())),
            audio: OnceLock::new(),
        });

        for track in tracks {
            let stream = track.render_stream(mapping, sample_rate);
            let progress = Arc::clone(&progress);
            let should_play = Arc::clone(&self.should_play);

            self.thread_pool
                .execute(render_job(stream, progress, should_play));
        }

        let old_progress = replace(&mut self.progress, progress);

        // Stop the threads that are rendering the old project
        old_progress.should_stop.set(true);
    }
}

fn render_job(
    stream: RenderStream,
    progress: Arc<Progress>,
    should_play: Arc<Cell<Option<ShouldPlay>>>,
) -> impl FnOnce() {
    let sample_rate = stream.sample_rate();

    move || {
        let mut audio = Audio::empty(sample_rate);

        for sample in stream {
            audio.samples.push(sample);

            if progress.should_stop.get() {
                return;
            }
        }

        let mut tracks = progress.unmastered_tracks.lock();
        tracks.push(audio);
        if tracks.len() == tracks.capacity() {
            let tracks = take(&mut *tracks);

            master(tracks, sample_rate, &progress, &should_play);
        }
    }
}

fn master(
    tracks: Vec<Audio>,
    sample_rate: SampleRate,
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
