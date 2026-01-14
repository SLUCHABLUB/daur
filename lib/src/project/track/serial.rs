//! Items pertaining to [`Serial`].

use crate::project::Track;
use crate::project::track::clip;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

/// The serial representation of a [track](Track).
#[derive(Serialize, Deserialize)]
pub(in crate::project) struct Serial<'data> {
    /// The name.
    pub name: Cow<'data, str>,
    /// The clips.
    pub clips: Vec<clip::Serial<'data>>,
}

impl<'data> From<&'data Track> for Serial<'data> {
    fn from(track: &'data Track) -> Self {
        let Track {
            id: _,
            name,
            clip_ids,
            clip_starts: _,
            clips,
        } = track;

        let name = Cow::Borrowed(name.as_str());

        // `clip_ids` is sorted in clip order which is why we iterate over it.
        let clips = clip_ids
            .iter()
            .map(|(position, id)| {
                let clip = &clips[id];

                clip::Serial::from_clip(*position, clip)
            })
            .collect();

        Serial { name, clips }
    }
}
