//! Types pertaining to [`Menu`].

use crate::view::{Direction, OnClick, View};
use crate::{Action, Popup, project};
use arcstr::{ArcStr, literal};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::path::Path;

const IMPORT_AUDIO: ArcStr = literal!("import audio");
const ADD_NOTES: ArcStr = literal!("add notes");
const OPEN_PIANO_ROLL: ArcStr = literal!("open piano roll");

/// Opens a popup for importing audio.
#[must_use]
pub fn open_import_audio_popup() -> Action {
    let action = move |file: &Path| Action::import_audio(file);

    Action::OpenPopup(Popup::explorer(IMPORT_AUDIO, action))
}

/// A context (right click) menu.
pub struct Menu {
    /// The buttons in the menu.
    pub buttons: Vec<(ArcStr, Action)>,
}

impl Menu {
    /// The context menu for the track overview.
    #[must_use]
    pub fn track_overview() -> Menu {
        Menu {
            buttons: vec![
                (IMPORT_AUDIO, open_import_audio_popup()),
                (ADD_NOTES, Action::Project(project::Action::AddNotes)),
                (OPEN_PIANO_ROLL, Action::OpenPianoRoll),
            ],
        }
    }

    /// Returns the view of the menu.
    pub fn view(&self) -> View {
        View::balanced_stack(
            Direction::Down,
            self.buttons.iter().map(|(label, action)| {
                View::simple_button(ArcStr::clone(label), OnClick::from(action.clone()))
            }),
        )
        .bordered()
    }
}

impl Debug for Menu {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();

        for (name, action) in &self.buttons {
            map.entry(name, action);
        }

        map.finish()
    }
}
