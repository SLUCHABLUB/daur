use crate::app::action::Action;
use crate::track::overview::open_import_audio_popup;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

pub fn default() -> HashMap<KeyEvent, Action> {
    HashMap::from([
        (
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            Action::Exit,
        ),
        (
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            Action::PlayPause,
        ),
        (
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            open_import_audio_popup(),
        ),
    ])
}
