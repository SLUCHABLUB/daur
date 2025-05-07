use crate::Track;
use crate::project::Project;
use crate::project::action::Action;
use crate::project::edit::Edit;
use crate::time::{Instant, NonZeroInstant};
use anyhow::Result;
use getset::Getters;
use std::sync::{Arc, Weak};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("No track is selected")]
struct NoTrackSelected;

/// Manages editing of a [project](Project).
#[derive(Debug, Getters)]
pub struct Manager {
    /// The project.
    #[get = "pub"]
    project: Project,
    // TODO: allow undoing
    history: Vec<Edit>,
}

impl Manager {
    /// Wraps a project in a new manager.
    #[must_use]
    pub fn new(project: Project) -> Manager {
        Manager {
            project,
            history: Vec::new(),
        }
    }

    /// Take an action on the project.
    ///
    /// # Errors
    ///
    /// If the action cannot be completed, a popup to open will be returned.
    pub fn take(
        &mut self,
        action: Action,
        cursor: Instant,
        selected_track: Weak<Track>,
    ) -> Result<()> {
        self.edit(Edit::from_action(action, cursor, selected_track)?)
    }

    fn edit(&mut self, edit: Edit) -> Result<()> {
        self.history.push(edit.clone());

        match edit {
            Edit::AddClip {
                track,
                position,
                clip,
            } => {
                self.project
                    .track_mut(&track)
                    .ok_or(NoTrackSelected)?
                    .clips
                    .insert(position, Arc::new(clip));
            }
            Edit::AddTrack(track) => self.project.tracks.push(Arc::new(track)),
            Edit::ChangeKey { position, key } => {
                if let Some(position) = NonZeroInstant::from_instant(position) {
                    Arc::make_mut(&mut self.project.key)
                        .changes
                        .insert(position, key);
                } else {
                    Arc::make_mut(&mut self.project.key).start = key;
                }
            }
        }

        Ok(())
    }
}
