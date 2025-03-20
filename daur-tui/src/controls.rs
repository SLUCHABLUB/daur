use crossterm::event::{KeyCode, KeyModifiers};
use daur::track::open_import_audio_popup;
use daur::{project, Action};
use std::collections::HashMap;

pub fn controls() -> HashMap<String, Action> {
    [
        ((KeyModifiers::CONTROL, KeyCode::Char('c')), Action::Exit),
        ((KeyModifiers::NONE, KeyCode::Char(' ')), Action::PlayPause),
        (
            (KeyModifiers::NONE, KeyCode::Char('i')),
            open_import_audio_popup(),
        ),
        (
            (KeyModifiers::NONE, KeyCode::Char('n')),
            Action::Project(project::Action::AddNotes),
        ),
        (
            (KeyModifiers::NONE, KeyCode::Char('p')),
            Action::OpenPianoRoll,
        ),
        ((KeyModifiers::NONE, KeyCode::Tab), Action::ScrollLeft),
        ((KeyModifiers::SHIFT, KeyCode::BackTab), Action::ScrollRight),
    ]
    .into_iter()
    .map(|((modifiers, code), action)| (format!("{modifiers} + {code}"), action))
    .collect()
}
