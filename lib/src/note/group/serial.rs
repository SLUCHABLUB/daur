use crate::Note;
use crate::metre::NonZeroDuration;
use crate::note;
use crate::note::InsertionError;
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

impl TryFrom<Serial> for note::Group {
    type Error = InsertionError;

    fn try_from(serial: Serial) -> Result<Self, Self::Error> {
        let Serial { duration, notes } = serial;

        let mut group = note::Group {
            notes: HashMap::new(),
            note_positions: HashMap::new(),
            duration,
        };

        for note in notes {
            group.try_insert(note.position, note.pitch, Note::from(note))?;
        }

        Ok(group)
    }
}
