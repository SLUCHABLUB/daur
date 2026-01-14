//! Items pertaining to [`Serial`].

use crate::audio;
use crate::metre::Instant;
use crate::note;
use crate::project::track::Clip;
use crate::project::track::clip;
use crate::ui::Colour;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

/// The serial representation of [`Clip`].
#[derive(Serialize, Deserialize)]
pub(in crate::project) struct Serial<'data> {
    /// The name.
    pub name: Cow<'data, str>,
    /// The position.
    pub position: Instant,

    /// The colour.
    pub colour: Colour,

    /// The content.
    pub content: SerialContent<'data>,
}

impl<'data> Serial<'data> {
    /// Constructs a new serial representation from a clip and position.
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

/// The serial representation of [`Content`].
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::project) enum SerialContent<'data> {
    /// An audio clip.
    Audio(Cow<'data, audio::FixedLength>),
    /// An note group.
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
