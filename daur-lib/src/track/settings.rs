use crate::app::Action;
use crate::track::Track;
use crate::view::{OnClick, ToText as _, View};
use arcstr::literal;

/// Returns the track settings.
pub fn settings(track: &Track, index: usize, selected: bool) -> View {
    let on_click = OnClick::from(Action::SelectTrack(index));

    literal!("TODO")
        .centred()
        .titled(track.name.clone())
        .with_thickness(selected)
        .on_click(on_click)
}
