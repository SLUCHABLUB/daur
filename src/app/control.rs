use crate::app::action::Action;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub const DEFAULT_CONTROLS: [(KeyEvent, Action); 2] = [
    (
        KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
        Action::Exit,
    ),
    (
        KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
        Action::PlayPause,
    ),
];
