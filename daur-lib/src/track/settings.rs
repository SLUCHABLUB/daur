use crate::app::Action;
use crate::track::Track;
use crate::view::{OnClick, ToText as _, View};
use alloc::sync::{Arc, Weak};
use arcstr::literal;

/// Returns the track settings.
pub(crate) fn settings(track: &Arc<Track>, selected: bool) -> View {
    let on_click = OnClick::from(Action::SelectClip {
        track: Arc::downgrade(track),
        clip: Weak::new(),
    });

    literal!("TODO")
        .centred()
        .bordered()
        .titled(track.name.clone())
        .with_thickness(selected)
        .on_click(on_click)
        .scrollable(Action::MoveOverview)
}
