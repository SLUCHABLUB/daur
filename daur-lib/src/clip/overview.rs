use crate::time::{Mapping, Period};
use crate::view::{OnClick, View};
use crate::{Action, Clip, Track};
use std::sync::{Arc, Weak};

/// Returns a view of a clip's overview.
pub(crate) fn overview(
    clip: Arc<Clip>,
    track: Weak<Track>,
    selected: bool,
    full_period: Period,
    visible_period: Period,
    mapping: Mapping,
) -> View {
    let title = clip.name.clone();
    let clip_reference = Arc::downgrade(&clip);

    View::canvas(clip.colour, move |context| {
        clip.content
            .paint_overview(context, full_period, visible_period, &mapping);
    })
    .titled(title)
    .with_thickness(selected)
    .on_click(OnClick::from(Action::SelectClip {
        track,
        clip: clip_reference,
    }))
}
