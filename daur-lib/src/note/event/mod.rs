mod sequence;
mod subsequence;

pub(crate) use sequence::Sequence;
pub(crate) use subsequence::Subsequence;

use crate::note::Pitch;
use crate::{Id, Note};

#[cfg_attr(doc, doc(hidden))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) enum Event {
    NoteOn { id: Id<Note>, pitch: Pitch },
    NoteOff(Id<Note>),
}
