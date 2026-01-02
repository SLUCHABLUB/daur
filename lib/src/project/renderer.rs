use crate::Audio;
use crate::Project;
use crate::UserInterface;
use crate::audio::Player;
use crate::audio::sample;
use crate::audio::sample::Instant;
use crate::node::Chain;
use crate::note::event::Sequence;
use crate::popup;
use crate::sync::Cell;
use crate::time;
use executors::Executor as _;
use executors::crossbeam_workstealing_pool::ThreadPool;
use executors::parker::DynParker;
use non_zero::non_zero;
use parking_lot::Mutex;
use saturating_cast::SaturatingCast as _;
use std::cmp::max;
use std::mem::replace;
use std::mem::take;
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) struct Renderer {
    /// The thread pool.
    ///
    /// This executor type is used due to this recommendation in the `executors` crate's readme:
    ///
    /// > If you don't know what hardware your code is going to run on, use the crossbeam_workstealing_pool
    thread_pool: ThreadPool<DynParker>,
    popups: Arc<popup::Manager>,
    /// Data that is shared across the workers in the tread pool.
    progress: Arc<Progress>,
}

struct Progress {
    should_stop: Cell<bool>,
    unmastered_tracks: Mutex<Vec<Audio>>,
    master: Mutex<Master>,
}

struct Play {
    from: time::Instant,
    player: Player,
}

enum Master {
    Finished(Audio),
    OnFinish {
        should_play: Option<Play>,
        should_export: Option<PathBuf>,
    },
}

impl Renderer {
    pub(crate) fn new(popups: Arc<popup::Manager>) -> Self {
        Renderer {
            thread_pool: ThreadPool::default(),
            popups,
            progress: Arc::new(Progress {
                should_stop: Cell::new(true),
                unmastered_tracks: Mutex::new(Vec::new()),
                master: Mutex::new(Master::Finished(Audio::empty(sample::Rate {
                    samples_per_second: non_zero!(1),
                }))),
            }),
        }
    }

    pub(crate) fn play_when_finished(&self, from: time::Instant, player: Player) {
        match &mut *self.progress.master.lock() {
            Master::Finished(audio) => {
                player.play(audio.clone(), from);
            }
            Master::OnFinish { should_play, .. } => {
                *should_play = Some(Play { from, player });
            }
        }
    }

    pub(crate) fn export_when_finished(&self, to: PathBuf) -> anyhow::Result<()> {
        match &mut *self.progress.master.lock() {
            Master::Finished(audio) => {
                audio.export(&to)?;
            }
            Master::OnFinish { should_export, .. } => {
                *should_export = Some(to);
            }
        }

        Ok(())
    }
}

impl Renderer {
    // TODO: the audio up to the point of the change may be reused
    pub(crate) fn restart<Ui: UserInterface>(
        &mut self,
        project: &Project,
        sample_rate: sample::Rate,
        ui: &'static Ui,
    ) -> anyhow::Result<()> {
        // Stop the threads that are rendering the old project
        self.progress.should_stop.set(true);

        let new_master = match replace(
            &mut *self.progress.master.lock(),
            Master::OnFinish {
                should_play: None,
                should_export: None,
            },
        ) {
            Master::OnFinish {
                should_play,
                should_export,
            } => Master::OnFinish {
                should_play,
                should_export,
            },
            Master::Finished(_) => Master::OnFinish {
                should_play: None,
                should_export: None,
            },
        };

        self.progress = Arc::new(Progress {
            should_stop: Cell::new(false),
            unmastered_tracks: Mutex::new(Vec::with_capacity(project.tracks.len())),
            master: Mutex::new(new_master),
        });

        let time_context = project.time_context();

        for track in project.tracks.values() {
            let audio = track.audio_sum(&time_context, sample_rate);
            let events = track.events(&time_context, sample_rate);

            // TODO: take from the track
            let chain = Chain::default();

            let progress = Arc::clone(&self.progress);
            let popups = Arc::clone(&self.popups);

            self.thread_pool
                .execute(move || render(&audio, &events, &chain, &progress, &popups, ui));
        }

        if project.tracks.is_empty() {
            master(Vec::new(), sample_rate, &self.progress)?;
        }

        Ok(())
    }
}

fn render<Ui: UserInterface>(
    input_audio: &Audio,
    events: &Sequence,
    chain: &Chain,
    progress: &Progress,
    popups: &popup::Manager,
    ui: &Ui,
) {
    try_render(input_audio, events, chain, progress)
        .unwrap_or_else(|error| popups.open(&error.into(), ui));
}

fn try_render(
    input_audio: &Audio,
    events: &Sequence,
    chain: &Chain,
    progress: &Progress,
) -> anyhow::Result<()> {
    let sample_rate = input_audio.sample_rate;

    // TODO: un-hardcode
    let batch_size = sample_rate.samples_per_second.get().saturating_cast();
    let batch_duration = sample::Duration {
        samples: batch_size,
    };

    let mut instance = chain.instantiate(sample_rate);

    let mut output_audio = Audio::empty(sample_rate);

    let input_end_point = max(
        Instant {
            since_start: input_audio.duration(),
        },
        events.last_timestamp().unwrap_or(Instant::START),
    );

    let mut position = Instant::START;
    let mut should_continue = position < input_end_point;

    while should_continue || position < input_end_point {
        let period = sample::Period {
            start: position,
            duration: batch_duration,
        };

        let audio = input_audio.subsection(period);
        let events = events.subsequence(period);

        let result = instance.process(batch_duration, audio, events);

        output_audio.superpose_with_offset(&result.audio, position.since_start);
        position += batch_duration;
        should_continue = result.should_continue;
    }

    output_audio.truncate_silence(input_audio.duration());

    let mut tracks = progress.unmastered_tracks.lock();
    tracks.push(output_audio);

    if tracks.len() == tracks.capacity() {
        let tracks = take(&mut *tracks);

        master(tracks, sample_rate, progress)?;
    }

    Ok(())
}

fn master(
    tracks: Vec<Audio>,
    sample_rate: sample::Rate,
    progress: &Progress,
) -> anyhow::Result<()> {
    let mut audio = Audio::empty(sample_rate);

    for track in tracks {
        audio.superpose(&track);
    }

    // TODO: apply the master plugin chain

    // TODO: truncate audio

    let mut audio_progress = progress.master.lock();

    if let Master::OnFinish {
        should_play,
        should_export,
    } = &*audio_progress
    {
        if let Some(Play { from, player }) = should_play {
            player.play(audio.clone(), *from);
        }

        if let Some(file) = should_export {
            audio.export(file)?;
        }
    }

    *audio_progress = Master::Finished(audio);

    Ok(())
}
