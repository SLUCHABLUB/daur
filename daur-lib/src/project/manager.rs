use crate::clip::Clip;
use crate::lock::Lock;
use crate::popup::Popup;
use crate::project::action::Action;
use crate::project::changing::Changing;
use crate::project::edit::Edit;
use crate::project::source::ProjectSource;
use crate::project::Project;
use crate::time::{Instant, NonZeroInstant, Signature, Tempo};
use crate::ui::{Grid, Length};
use crate::widget::Widget;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("No track is selected")]
struct NoTrackSelected;

#[derive(Debug)]
pub struct Manager {
    project: Lock<Project>,
    // TODO: allow undoing
    history: Lock<Vec<Edit>>,
}

impl Manager {
    pub fn new(project: Project) -> Manager {
        Manager {
            project: Lock::new(project),
            history: Lock::new(Vec::new()),
        }
    }

    pub fn tempo(&self) -> Arc<Changing<Tempo>> {
        Arc::clone(&self.project.read().tempo)
    }

    pub fn time_signature(&self) -> Arc<Changing<Signature>> {
        Arc::clone(&self.project.read().time_signature)
    }

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

    pub fn source(&self, sample_rate: u32, cursor: Instant) -> ProjectSource {
        self.project.read().to_source(sample_rate, cursor)
    }

    pub fn bar(&self, playing: bool) -> impl Widget {
        self.project.read().bar(playing)
    }

    pub fn workspace(
        &self,
        track_settings_size: Length,
        grid: Grid,
        overview_offset: Length,
        selected_track_index: usize,
        selected_clip_index: usize,
        cursor: Instant,
    ) -> impl Widget {
        self.project.read().workspace(
            track_settings_size,
            grid,
            overview_offset,
            selected_track_index,
            selected_clip_index,
            cursor,
        )
    }

    pub fn handle(
        &self,
        action: Action,
        cursor: Instant,
        selected_track: usize,
    ) -> Result<(), Arc<Popup>> {
        self.edit(Edit::from_action(action, cursor, selected_track)?)
    }

    fn edit(&self, edit: Edit) -> Result<(), Arc<Popup>> {
        let mut project = self.project.write();

        // TODO: is the guard dropped here?
        self.history.write().push(edit.clone());

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
