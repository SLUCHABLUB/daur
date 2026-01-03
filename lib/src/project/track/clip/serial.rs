use crate::audio;
use crate::metre::Instant;
use crate::note;
use crate::project::track::Clip;
use crate::project::track::clip;
use crate::ui::Colour;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub(in crate::project) struct Serial<'data> {
    pub name: Cow<'data, str>,
    pub position: Instant,

    pub colour: Colour,

    pub content: SerialContent<'data>,
}

impl<'data> Serial<'data> {
    pub(crate) fn from_clip(position: Instant, clip: &'data Clip) -> Self {
        let Clip {
            id: _,
            name,
            colour,
            content,
        } = clip;

        Serial {
            name: Cow::Borrowed(name.as_str()),
            position,
            colour: *colour,
            content: SerialContent::from(content),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::project) enum SerialContent<'data> {
    Audio(Cow<'data, audio::FixedLength>),
    Notes(note::group::Serial),
}

impl<'data> From<&'data clip::Content> for SerialContent<'data> {
    fn from(content: &'data clip::Content) -> Self {
        match content {
            clip::Content::Audio(audio) => SerialContent::Audio(Cow::Borrowed(audio)),
            clip::Content::Notes(notes) => SerialContent::Notes(note::group::Serial::from(notes)),
        }
    }
}
