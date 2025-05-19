use crate::Id;
use crate::app::Action;
use crate::project::Track;
use crate::view::{OnClick, ToText as _, View};
use arcstr::literal;

/// Returns the track settings.
pub(crate) fn settings(track: &Track, selected: bool) -> View {
    let on_click = OnClick::from(Action::SelectClip {
        track: track.id,
        clip: Id::NONE,
    });

    literal!("TODO")
        .centred()
        .bordered()
        .titled(track.name.clone())
        .with_thickness(selected)
        .on_click(on_click)
        .scrollable(Action::MoveOverview)
}
