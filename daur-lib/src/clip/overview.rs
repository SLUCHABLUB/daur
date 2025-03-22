use crate::time::{Mapping, Period};
use crate::view::{OnClick, View};
use crate::{Action, Clip};
use std::sync::Arc;

/// Returns a view of the overview of a clip.
pub fn overview(
    clip: Arc<Clip>,
    track_index: usize,
    index: usize,
    selected: bool,
    full_period: Period,
    visible_period: Period,
    mapping: Mapping,
) -> View {
    let title = clip.name.clone();

    View::canvas(clip.colour, move |context| {
        clip.content
            .paint_overview(context, full_period, visible_period, &mapping);
    })
    .titled(title)
    .with_thickness(selected)
    .on_click(OnClick::from(Action::SelectClip { track_index, index }))
}
