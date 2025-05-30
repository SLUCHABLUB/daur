use crate::project::Track;
use crate::project::track::Clip;
use crate::ui::{Grid, Length};
use crate::view::{SelectableItem, View};
use crate::{Id, project};

/// Returns a view of a clip's overview.
pub(crate) fn overview(
    clip: &Clip,
    track: Id<Track>,
    selected: bool,
    project_settings: &project::Settings,
    grid: Grid,
    crop_start: Length,
) -> View {
    View::canvas(
        clip.colour,
        clip.content
            .overview_painter(project_settings, grid, crop_start),
    )
    .titled(clip.name())
    .with_thickness(selected)
    .selectable(SelectableItem::Clip {
        track,
        clip: clip.id,
    })
}
