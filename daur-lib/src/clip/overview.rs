use crate::metre::Period;
use crate::project::Settings;
use crate::ui::Grid;
use crate::view::{OnClick, View};
use crate::{Action, Clip, Track};
use alloc::sync::{Arc, Weak};

/// Returns a view of a clip's overview.
pub(crate) fn overview(
    clip: Arc<Clip>,
    track: Weak<Track>,
    selected: bool,
    full_period: Period,
    visible_period: Period,
    settings: &Settings,
    grid: Grid,
) -> View {
    let title = clip.name.clone();
    let clip_reference = Arc::downgrade(&clip);
    let settings = settings.clone();

    View::canvas(clip.colour, move |context| {
        clip.content
            .paint_overview(context, full_period, visible_period, &settings, grid);
    })
    .titled(title)
    .with_thickness(selected)
    .on_click(OnClick::from(Action::SelectClip {
        track,
        clip: clip_reference,
    }))
}
