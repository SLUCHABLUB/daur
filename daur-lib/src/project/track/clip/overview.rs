use crate::Id;
use crate::metre::OffsetMapping;
use crate::project::Track;
use crate::project::track::Clip;
use crate::ui::Length;
use crate::view::{SelectableItem, View};

/// Returns a view of a clip's overview.
pub(crate) fn overview(
    clip: &Clip,
    track: Id<Track>,
    selected: bool,
    offset_mapping: OffsetMapping,
    crop_start: Length,
) -> View {
    View::canvas(
        clip.colour,
        clip.content.overview_painter(offset_mapping, crop_start),
    )
    .titled(clip.name())
    .with_thickness(selected)
    .selectable(SelectableItem::Clip {
        track,
        clip: clip.id,
    })
}
