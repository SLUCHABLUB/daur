use crate::project::Track;
use crate::project::track::clip;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub(in crate::project) struct Serial<'data> {
    pub name: Cow<'data, str>,
    pub clips: Vec<clip::Serial<'data>>,
}

impl<'data> Serial<'data> {
    pub(crate) fn from_track(track: &'data Track) -> Self {
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
