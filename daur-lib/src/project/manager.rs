use crate::audio::Player;
use crate::clip::Clip;
use crate::key::Key;
use crate::lock::Lock;
use crate::project::action::Action;
use crate::project::edit::Edit;
use crate::project::{Project, bar, workspace};
use crate::time::{Instant, NonZeroInstant, Signature, Tempo};
use crate::ui::{Grid, Length, NonZeroLength};
use crate::view::View;
use crate::{Changing, Track, UserInterface, time, ui};
use anyhow::Result;
use std::sync::{Arc, Weak};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("No track is selected")]
struct NoTrackSelected;

/// Manages mutation of a [project](Project).
#[derive(Debug)]
pub struct Manager {
    project: Lock<Project>,
    // TODO: allow undoing
    history: Lock<Vec<Edit>>,
}

impl Manager {
    /// Wraps a project in a new manager.
    #[must_use]
    pub fn new(project: Project) -> Manager {
        Manager {
            project: Lock::new(project),
            history: Lock::new(Vec::new()),
        }
    }

    pub(crate) fn tracks(&self) -> Vec<Arc<Track>> {
        self.project.read().tracks.clone()
    }

    /// Returns the key of the project.
    #[must_use]
    pub fn key(&self) -> Arc<Changing<Key>> {
        self.project.read().key()
    }

    /// Returns the tempo of the project.
    #[must_use]
    pub fn tempo(&self) -> Arc<Changing<Tempo>> {
        self.project.read().tempo()
    }

    /// Returns the time signature of the project.
    #[must_use]
    pub fn time_signature(&self) -> Arc<Changing<Signature>> {
        self.project.read().time_signature()
    }

    /// Returns the [time mapping](time::Mapping) for the project.
    #[must_use]
    pub fn time_mapping(&self) -> time::Mapping {
        time::Mapping {
            tempo: self.tempo(),
            time_signature: self.time_signature(),
        }
    }

    /// Returns the [UI mapping](ui::Mapping) for the project.
    #[must_use]
    pub fn ui_mapping(&self, grid: Grid) -> ui::Mapping {
        ui::Mapping {
            time_signature: self.time_signature(),
            grid,
        }
    }

    pub(crate) fn bar<Ui: UserInterface>(&self, playing: bool) -> View {
        let project = self.project.read();
        bar::<Ui>(
            project.title(),
            project.tempo.start,
            project.time_signature.start,
            project.key.start,
            playing,
        )
    }

    // TODO: merge `overview_offset` and `track_settings_width` into temporary settings and remove expect
    #[expect(clippy::too_many_arguments, reason = "todo")]
    pub(crate) fn workspace<Ui: UserInterface>(
        &self,
        track_settings_size: NonZeroLength,
        grid: Grid,
        overview_offset: Length,
        selected_track: &Weak<Track>,
        selected_clip: &Weak<Clip>,
        cursor: Instant,
        player: Option<&Player>,
    ) -> View {
        let project = self.project.read();

        workspace::<Ui>(
            overview_offset,
            selected_track,
            selected_clip,
            track_settings_size,
            project.tracks.clone(),
            &project.time_mapping(),
            project.ui_mapping(grid),
            cursor,
            player,
        )
    }

    /// Take an action on the project.
    ///
    /// # Errors
    ///
    /// If the action cannot be completed, a popup to open will be returned.
    pub fn take(&self, action: Action, cursor: Instant, selected_track: Weak<Track>) -> Result<()> {
        self.edit(Edit::from_action(action, cursor, selected_track)?)
    }

    fn edit(&self, edit: Edit) -> Result<()> {
        self.history.write().push(edit.clone());

        let mut project = self.project.write();

        match edit {
            Edit::AddClip {
                track,
                position,
                clip,
            } => {
                project
                    .track_mut(&track)
                    .ok_or(NoTrackSelected)?
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
