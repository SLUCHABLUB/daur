//! Items pertaining to [`Track`].

mod action;
pub mod clip;
mod overview;
mod settings;

pub use action::Action;
#[doc(inline)]
pub use clip::Clip;

pub(crate) use overview::overview;
pub(crate) use settings::settings;

use crate::audio::sample::Pair;
use crate::audio::{NonEmpty, sample};
use crate::metre::{Duration, Instant, NonZeroDuration};
use crate::note::Event;
use crate::{Audio, Id, NonZeroRatio, Selection, project};
use anyhow::{Result, bail};
use arcstr::{ArcStr, literal};
use getset::{CopyGetters, Getters, MutGetters};
use hound::WavReader;
use indexmap::IndexMap;
use non_zero::non_zero;
use sorted_vec::SortedVec;
use std::collections::{BTreeMap, HashMap};
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use thiserror::Error;

const DEFAULT_TITLE: ArcStr = literal!("a track");

const DEFAULT_NOTES_DURATION: NonZeroDuration = NonZeroDuration {
    whole_notes: NonZeroRatio::integer(non_zero!(4)),
};

#[derive(Debug, Error)]
#[error("no clip is selected")]
struct NoClipSelected;

#[derive(Debug, Error)]
#[error("there is already a clip at that position")]
struct InsertClipError;

#[derive(Debug, Error)]
#[error("the audio format format `{}` is not (yet) supported", format.to_string_lossy())]
struct UnsupportedFormatError {
    format: OsString,
}

#[derive(Debug, Error)]
#[error("unable to infer the audio format of the file `{file}`")]
struct NoExtensionError {
    file: PathBuf,
}

#[derive(Debug, Error)]
#[error("cannot insert an empty audio file")]
struct EmptyAudioFile;

/// A musical track.
// TODO: Test that this isn't `Clone` (bc. id).
#[cfg_attr(doc, doc(hidden))]
#[derive(Debug, Getters, MutGetters, CopyGetters)]
pub struct Track {
    #[get_copy = "pub(super)"]
    id: Id<Track>,
    /// The name of the track.
    name: ArcStr,
    // TODO: use `Dimap<Instant, Id<Clip>, Clip, Bi<Btree, StdHash>, Index>`
    /// The clips in the track.
    clip_ids: BTreeMap<Instant, Id<Clip>>,
    clip_starts: HashMap<Id<Clip>, Instant>,
    clips: IndexMap<Id<Clip>, Clip>,
}

impl Track {
    /// Constructs a new, empty, track.
    #[must_use]
    pub(crate) fn new() -> Track {
        Track {
            id: Id::generate(),
            name: DEFAULT_TITLE,
            clip_ids: BTreeMap::new(),
            clip_starts: HashMap::new(),
            clips: IndexMap::new(),
        }
    }

    /// Returns a reference to a clip.
    #[must_use]
    pub(crate) fn clip(&self, id: Id<Clip>) -> Option<(Instant, &Clip)> {
        let clip = self.clips.get(&id)?;
        let start = self.clip_starts.get(&id)?;
        Some((*start, clip))
    }

    fn minimum_duration(&self, settings: &project::Settings) -> Duration {
        let Some((start, clip_id)) = self.clip_ids.last_key_value() else {
            return Duration::ZERO;
        };

        let Some(clip) = self.clips.get(clip_id) else {
            return Duration::ZERO;
        };

        clip.period(*start, settings).get().end().since_start
    }

    pub(crate) fn audio_sum(
        &self,
        settings: &project::Settings,
        sample_rate: sample::Rate,
    ) -> Audio {
        let minimum_end = Instant {
            since_start: self.minimum_duration(settings),
        };
        let minimum_end = minimum_end.to_real_time(settings);
        let minimum_end = minimum_end.since_start * sample_rate;
        let minimum_sample_count = minimum_end.samples;

        let mut audio = Audio {
            sample_rate,
            samples: vec![Pair::ZERO; minimum_sample_count],
        };

        for (start, clip_id) in &self.clip_ids {
            let Some(clip) = self.clips.get(clip_id) else {
                continue;
            };

            if let Some(clip) = clip.content().as_audio() {
                let clip_start = start.to_real_time(settings) * sample_rate;

                audio.add_assign_at(clip.as_audio(), clip_start.since_start);
            }
        }

        audio
    }

    pub(crate) fn events(
        &self,
        settings: &project::Settings,
        sample_rate: sample::Rate,
    ) -> SortedVec<Event> {
        let mut events = SortedVec::new();

        for (start, clip_id) in &self.clip_ids {
            let Some(clip) = self.clips.get(clip_id) else {
                continue;
            };

            events.extend(clip.events(*start, settings, sample_rate));
        }

        events
    }

    fn try_insert_clip(&mut self, position: Instant, clip: Clip) -> Result<()> {
        if self.clip_ids.contains_key(&position) {
            bail!(InsertClipError);
        }

        // TODO: check for overlap

        self.clip_ids.insert(position, clip.id());
        self.clip_starts.insert(clip.id(), position);
        self.clips.insert(clip.id(), clip);

        Ok(())
    }

    #[remain::check]
    pub(super) fn take_action(
        &mut self,
        action: Action,
        cursor: Instant,
        selection: &mut Selection,
    ) -> Result<()> {
        #[sorted]
        match action {
            Action::AddNotes => {
                self.try_insert_clip(cursor, Clip::empty_notes(DEFAULT_NOTES_DURATION))
            }
            Action::Clip(action) => {
                let clip = self
                    .clips
                    .get_mut(&selection.clip())
                    .ok_or(NoClipSelected)?;

                let clip_start = *self.clip_starts.get(&clip.id()).ok_or(NoClipSelected)?;

                clip.take_action(clip_start, action)
            }
            Action::ImportAudio { file } => {
                let Some(extension) = file.extension() else {
                    bail!(NoExtensionError { file });
                };

                // TODO: look at the symphonia crate
                let audio = match extension.to_string_lossy().as_ref() {
                    "wav" | "wave" => {
                        let reader = WavReader::open(&file)?;
                        Audio::try_from(reader)?
                    }
                    _ => {
                        bail!(UnsupportedFormatError {
                            format: extension.to_owned(),
                        });
                    }
                };

                let audio = NonEmpty::from_audio(audio).ok_or(EmptyAudioFile)?;

                let name = file
                    .file_stem()
                    .map(OsStr::to_string_lossy)
                    .map(ArcStr::from)
                    .unwrap_or_default();

                self.try_insert_clip(cursor, Clip::from_audio(name, audio))
            }
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}
