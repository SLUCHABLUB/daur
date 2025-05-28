use crossterm::event::{KeyCode, KeyModifiers};
use daur::app::Action;
use daur::project;
use daur::project::track;
use daur::view::context::open_import_audio_popup;
use std::collections::HashMap;

pub(crate) fn controls() -> HashMap<(KeyModifiers, KeyCode), Action> {
    HashMap::from([
        ((KeyModifiers::CONTROL, KeyCode::Char('c')), Action::Exit),
        (
            (KeyModifiers::NONE, KeyCode::Char(' ')),
            Action::TogglePlayback,
        ),
        (
            (KeyModifiers::NONE, KeyCode::Char('i')),
            open_import_audio_popup(),
        ),
        (
            (KeyModifiers::NONE, KeyCode::Char('n')),
            Action::Project(project::Action::Track(track::Action::AddNotes)),
        ),
        (
            (KeyModifiers::NONE, KeyCode::Char('p')),
            Action::TogglePianoRoll,
        ),
    ])
}
