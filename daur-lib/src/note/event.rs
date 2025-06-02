use clack_host::events::UnknownEvent;
use clack_host::events::event_types::{NoteOffEvent, NoteOnEvent};
use std::cmp::Ordering;

#[derive(Debug)]
pub(crate) enum Event {
    NoteOn(NoteOnEvent),
    NoteOff(NoteOffEvent),
}

impl AsRef<UnknownEvent> for Event {
    fn as_ref(&self) -> &UnknownEvent {
        match self {
            Event::NoteOn(event) => event.as_ref(),
            Event::NoteOff(event) => event.as_ref(),
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        u32::cmp(
            &self.as_ref().header().time(),
            &other.as_ref().header().time(),
        )
    }
}
