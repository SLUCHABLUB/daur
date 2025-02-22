use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Key {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl Key {
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Key {
        Key { code, modifiers }
    }

    pub fn from_event(
        KeyEvent {
            code,
            modifiers,
            kind,
            state: _,
        }: KeyEvent,
    ) -> Option<Key> {
        match kind {
            KeyEventKind::Release => return None,
            KeyEventKind::Press | KeyEventKind::Repeat => (),
        }

        Some(Key::new(code, modifiers))
    }

    pub fn to_event(self) -> Event {
        Event::Key(KeyEvent::new(self.code, self.modifiers))
    }
}
