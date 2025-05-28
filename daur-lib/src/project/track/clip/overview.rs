use crate::app::Action;
use crate::project::Track;
use crate::project::track::Clip;
use crate::ui::{Grid, Length};
use crate::view::{OnClick, View};
use crate::{Id, project};
use closure::closure;

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
        closure!([clone clip.content, clone project_settings] move |context| {
            content.paint_overview(context, &project_settings, grid, crop_start);
        }),
    )
    .titled(clip.name())
    .with_thickness(selected)
    .on_click(OnClick::from(Action::SelectClip {
        track,
        clip: clip.id,
    }))
}
