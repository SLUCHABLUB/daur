use crate::Holdable;
use crate::Id;
use crate::Selectable;
use crate::View;
use crate::metre::OffsetMapping;
use crate::project::Track;
use crate::project::track::Clip;
use crate::project::track::clip::Path;
use crate::ui::Length;

/// Returns a view of a clip's overview.
pub(crate) fn overview(
    clip: &Clip,
    selected: bool,
    offset_mapping: OffsetMapping,
    crop_start: Length,
    track: Id<Track>,
) -> View {
    let path = Path::new(track, clip.id);

    View::y_stack([
        View::TitleBar {
            title: clip.name(),
            highlighted: selected,
        }
        .grabbable(move |_| Some(Holdable::Clip(path)))
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
