use crate::app::OverviewSettings;
use crate::clip::Clip;
use crate::length::Length;
use crate::lock::Lock;
use crate::popup::Popup;
use crate::project::action::Action;
use crate::project::changing::Changing;
use crate::project::edit::Edit;
use crate::project::source::ProjectSource;
use crate::project::Project;
use crate::time::instant::Instant;
use crate::time::tempo::Tempo;
use crate::time::TimeSignature;
use crate::widget::Widget;
use std::sync::{Arc, Weak};
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

    pub fn time_signature(&self) -> Arc<Changing<TimeSignature>> {
        Arc::clone(&self.project.read().time_signature)
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
        overview_settings: OverviewSettings,
        selected_track_index: usize,
        selected_clip: &Weak<Clip>,
        cursor: Instant,
    ) -> impl Widget {
        self.project.read().workspace(
            track_settings_size,
            overview_settings,
            selected_track_index,
            selected_clip,
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
                if position == Instant::START {
                    Arc::make_mut(&mut project.key).start = key;
                } else {
                    Arc::make_mut(&mut project.key)
                        .changes
                        .insert(position, key);
                }
            }
        }

        Ok(())
    }
}
