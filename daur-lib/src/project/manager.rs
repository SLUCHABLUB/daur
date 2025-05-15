use crate::Clip;
use crate::app::Selection;
use crate::metre::{Instant, NonZeroDuration, NonZeroInstant};
use crate::project::Project;
use crate::project::action::Action;
use crate::project::edit::Edit;
use alloc::sync::Arc;
use anyhow::Result;
use getset::Getters;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("no track is selected")]
struct NoTrackSelected;

#[derive(Debug, Error)]
#[error("no clip is selected")]
struct NoClipSelected;

#[derive(Debug, Error)]
#[error("the selected clip is not a notes-clip")]
struct NoNotesSelected;

#[derive(Debug, Error)]
#[error("there is already a clip at that position")]
struct InsertClipError(Arc<Clip>);

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
    pub fn take(&mut self, action: Action, cursor: Instant, selection: &Selection) -> Result<()> {
        self.edit(Edit::from_action(action, cursor, selection)?)
    }

    fn edit(&mut self, edit: Edit) -> Result<()> {
        self.history.push(edit.clone());

        match edit {
            Edit::AddNote {
                track,
                clip,
                position: note_position,
                pitch,
                mut note,
            } => {
                let (clip_position, clip) = self
                    .project
                    .track_mut(&track)
                    .ok_or(NoTrackSelected)?
                    .clip_mut(&clip)
                    .ok_or(NoClipSelected)?;

                if note_position < clip_position {
                    let difference = clip_position - note_position;
                    let Some(duration) =
                        NonZeroDuration::from_duration(note.duration.get() - difference)
                    else {
                        return Ok(());
                    };

                    note.duration = duration;
                }

                let relative_position = note_position - clip_position.since_start;

                clip.content
                    .as_notes_mut()
                    .ok_or(NoNotesSelected)?
                    .try_insert(relative_position, pitch, note);
            }
            Edit::AddClip {
                track,
                position,
                clip,
            } => {
                self.project
                    .track_mut(&track)
                    .ok_or(NoTrackSelected)?
                    .clips
                    .try_insert(position, Arc::new(clip))
                    .map_err(InsertClipError)?;
            }
            Edit::AddTrack(track) => self.project.tracks.push(Arc::new(track)),
            Edit::ChangeKey { position, key } => {
                if let Some(position) = NonZeroInstant::from_instant(position) {
                    Arc::make_mut(&mut self.project.settings.key)
                        .changes
                        .insert(position, key);
                } else {
                    Arc::make_mut(&mut self.project.settings.key).start = key;
                }
            }
        }

        Ok(())
    }
}
