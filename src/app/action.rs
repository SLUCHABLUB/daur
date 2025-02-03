use crate::app::reference::AppShare;
use crate::track::Track;
use std::time::SystemTime;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    /// Add an empty track
    AddTrack,
    /// Save and exit the program
    Exit,
    /// Stop playing
    Pause,
    /// Start playing
    Play,
    /// `Play` or `Pause`
    PlayPause,
    // TODO: add scripting
}

impl Action {
    pub fn take(self, app: &AppShare) {
        // TODO: add to action tree
        match self {
            Action::AddTrack => {
                let mut app = app.write_lock();
                app.project.tracks.push(Track::default());
                app.selected_track = Some(app.project.tracks.len() - 1);
            }
            Action::Exit => app.set_exit(),
            Action::Pause => {
                let mut app = app.write_lock();
                app.cursor = app.playback_position();
                app.playback_start = None;
            }
            Action::Play => {
                let mut app = app.write_lock();
                app.playback_start = Some(SystemTime::now());
            }
            Action::PlayPause => {
                let mut app = app.write_lock();
                if app.playback_start.is_some() {
                    app.cursor = app.playback_position();
                    app.playback_start = None;
                } else {
                    app.playback_start = Some(SystemTime::now());
                }
            }
        }
    }
}
