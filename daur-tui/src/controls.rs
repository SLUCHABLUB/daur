use crossterm::event::{KeyCode, KeyModifiers};
use daur::app::Action;
use daur::popup::Specification;
use daur::project::Edit;
use std::collections::HashMap;

pub(crate) fn controls() -> HashMap<(KeyModifiers, KeyCode), Action> {
    HashMap::from([
        (
            (KeyModifiers::NONE, KeyCode::Char(' ')),
            Action::TogglePlayback,
        ),
        ((KeyModifiers::CONTROL, KeyCode::Char('q')), Action::Exit),
        (
            (KeyModifiers::NONE, KeyCode::Char('e')),
            Action::ToggleEditMode,
        ),
        (
            (KeyModifiers::NONE, KeyCode::Char('x')),
            Action::ExportProject,
        ),
        (
            (KeyModifiers::NONE, KeyCode::Char('i')),
            Action::OpenPopup(Specification::AudioImporter),
        ),
        (
            (KeyModifiers::NONE, KeyCode::Char('n')),
            Action::Edit(Edit::AddNoteGroup),
        ),
        (
            (KeyModifiers::NONE, KeyCode::Char('p')),
            Action::TogglePianoRoll,
        ),
        (
            (KeyModifiers::NONE, KeyCode::Backspace),
            Action::Edit(Edit::Delete),
        ),
    ])
}
