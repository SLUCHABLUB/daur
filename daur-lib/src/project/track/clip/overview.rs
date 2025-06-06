use crate::metre::OffsetMapping;
use crate::project::Track;
use crate::project::track::Clip;
use crate::project::track::clip::Path;
use crate::ui::Length;
use crate::{Id, Selectable, View};

/// Returns a view of a clip's overview.
pub(crate) fn overview(
    clip: &Clip,
    selected: bool,
    offset_mapping: OffsetMapping,
    crop_start: Length,
    track: Id<Track>,
) -> View {
    View::y_stack([
        View::TitleBar {
            title: clip.name(),
            highlighted: selected,
        }
        .quotated_minimally(),
        View::canvas(
            clip.colour,
            clip.content.overview_painter(offset_mapping, crop_start),
        )
        .fill_remaining(),
    ])
    .selectable(Selectable::Clip(Path {
        track,
        clip: clip.id,
    }))
}
