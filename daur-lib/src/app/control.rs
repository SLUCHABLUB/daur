use crate::app::action::Action;
use crate::keyboard::Key;
use crate::track::overview::open_import_audio_popup;
use crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;

pub fn default() -> HashMap<Key, Action> {
    HashMap::from([
        (
            Key::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            Action::Exit,
        ),
        (
            Key::new(KeyCode::Char(' '), KeyModifiers::NONE),
            Action::PlayPause,
        ),
        (
            Key::new(KeyCode::Char('i'), KeyModifiers::NONE),
            open_import_audio_popup(),
        ),
    ])
}
