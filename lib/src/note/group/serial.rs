use crate::Note;
use crate::metre::NonZeroDuration;
use crate::note;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;
use std::collections::HashMap;

impl Serialize for note::Group {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Serial::from(self).serialize(serializer)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Serial {
    pub duration: NonZeroDuration,
    pub notes: BTreeSet<note::Serial>,
}

impl From<&note::Group> for Serial {
    fn from(group: &note::Group) -> Self {
        Serial {
            duration: group.duration(),
            notes: group
                .notes
                .iter()
                .map(|((position, pitch), note)| note::Serial {
                    position: *position,
                    pitch: *pitch,
                    duration: note.duration,
                })
                .collect(),
        }
    }
}

impl From<Serial> for note::Group {
    fn from(serial: Serial) -> Self {
        let Serial { duration, notes } = serial;

        let mut note_positions = HashMap::new();

        let notes = notes
            .into_iter()
            .map(|note| {
                let position = (note.position, note.pitch);
                let note = Note::from(note);

                note_positions.insert(note.id(), position);

                (position, note)
            })
            .collect();

        note::Group {
            notes,
            note_positions,
            duration,
        }
    }
}
