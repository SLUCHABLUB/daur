use crate::project::Settings;
use crate::ui::{Grid, Length};
use crate::view::{OnClick, View};
use crate::{Action, Clip, Id, Track};
use closure::closure;

/// Returns a view of a clip's overview.
pub(crate) fn overview(
    clip: &Clip,
    track: Id<Track>,
    selected: bool,
    settings: &Settings,
    grid: Grid,
    crop_start: Length,
) -> View {
    View::canvas(
        clip.colour,
        closure!([clone clip.content, clone settings] move |context| {
            content.paint_overview(context, &settings, grid, crop_start);
        }),
    )
    .titled(clip.name())
    .with_thickness(selected)
    .on_click(OnClick::from(Action::SelectClip {
        track,
        clip: clip.id,
    }))
}
