mod sequence;
mod subsequence;

pub(crate) use sequence::Sequence;
pub(crate) use subsequence::Subsequence;

use crate::note;
use crate::note::Pitch;

#[cfg_attr(doc, doc(hidden))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) enum Event {
    NoteOn { id: note::Id, pitch: Pitch },
    NoteOff(note::Id),
}
