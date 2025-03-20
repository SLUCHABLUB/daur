use crate::clip::Clip;
use crate::key::Key;
use crate::lock::Lock;
use crate::popup::Popup;
use crate::project::action::Action;
use crate::project::edit::Edit;
use crate::project::source::ProjectSource;
use crate::project::{bar, workspace, Project};
use crate::time::{Instant, NonZeroInstant, Signature, Tempo};
use crate::ui::{Grid, Length, Offset};
use crate::view::View;
use crate::Changing;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("No track is selected")]
struct NoTrackSelected;

/// Manages mutation of a [`Project`].
#[derive(Debug)]
pub struct Manager {
    project: Lock<Project>,
    // TODO: allow undoing
    history: Lock<Vec<Edit>>,
}

impl Manager {
    /// Constructs a new manager for `project`.
    #[must_use]
    pub fn new(project: Project) -> Manager {
        Manager {
            project: Lock::new(project),
            history: Lock::new(Vec::new()),
        }
    }

    /// Returns the key of the project.
    #[must_use]
    pub fn key(&self) -> Arc<Changing<Key>> {
        Arc::clone(&self.project.read().key)
    }

    /// Returns the tempo of the project.
    #[must_use]
    pub fn tempo(&self) -> Arc<Changing<Tempo>> {
        Arc::clone(&self.project.read().tempo)
    }

    /// Returns the time signature of the project.
    #[must_use]
    pub fn time_signature(&self) -> Arc<Changing<Signature>> {
        Arc::clone(&self.project.read().time_signature)
    }

    /// Returns a clip from its index.
    #[must_use]
    pub fn clip(
        &self,
        selected_track_index: usize,
        selected_clip_index: usize,
    ) -> Option<Arc<Clip>> {
        self.project
            .read()
            .tracks
            .get(selected_track_index)?
            .clips
            .values()
            .nth(selected_clip_index)
            .map(Arc::clone)
    }

    /// Returns an audio source for the project
    #[must_use]
    pub fn source(&self, sample_rate: u32, cursor: Instant) -> ProjectSource {
        let tracks = self.project.read().tracks.clone();
        let mapping = self.project.read().time_mapping();
        let offset = cursor.to_sample(&mapping, sample_rate);
        ProjectSource {
            sample_rate,
            tracks: tracks
                .into_iter()
                .map(|track| track.to_source(&mapping, sample_rate, offset))
                .collect(),
        }
    }

    pub(crate) fn bar(&self, playing: bool) -> View {
        let project = self.project.read();
        bar(
            project.title(),
            project.tempo.start,
            project.time_signature.start,
            project.key.start,
            playing,
        )
    }

    pub(crate) fn workspace(
        &self,
        track_settings_size: Length,
        grid: Grid,
        overview_offset: Offset,
        selected_track_index: usize,
        selected_clip_index: usize,
        cursor: Instant,
    ) -> View {
        let project = self.project.read();

        workspace(
            overview_offset,
            selected_track_index,
            selected_clip_index,
            track_settings_size,
            project.tracks.clone(),
            project.time_mapping(),
            project.ui_mapping(grid),
            cursor,
        )
    }

    /// Take an action on the project.
    ///
    /// # Errors
    ///
    /// If the action can not be completed, a popup to open will be returned.
    pub fn take(
        &self,
        action: Action,
        cursor: Instant,
        selected_track: usize,
    ) -> Result<(), Arc<Popup>> {
        self.edit(Edit::from_action(action, cursor, selected_track)?)
    }

    fn edit(&self, edit: Edit) -> Result<(), Arc<Popup>> {
        self.history.write().push(edit.clone());

        let mut project = self.project.write();

        match edit {
            Edit::AddClip {
                track,
                position,
                clip,
            } => {
                Arc::make_mut(
                    project
                        .tracks
                        .get_mut(track)
                        .ok_or(Popup::error(NoTrackSelected))?,
                )
                .clips
                .insert(position, Arc::new(clip));
            }
            Edit::AddTrack(track) => project.tracks.push(Arc::new(track)),
            Edit::ChangeKey { position, key } => {
                if let Some(position) = NonZeroInstant::from_instant(position) {
                    Arc::make_mut(&mut project.key)
                        .changes
                        .insert(position, key);
                } else {
                    Arc::make_mut(&mut project.key).start = key;
                }
            }
        }

        Ok(())
    }
}
