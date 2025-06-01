use crate::Selectable;
use crate::app::Action;
use crate::project::Track;
use crate::view::{ToText as _, View};
use arcstr::literal;

/// Returns the track settings.
pub(crate) fn settings(track: &Track, selected: bool) -> View {
    literal!("TODO")
        .centred()
        .bordered()
        .titled(track.name.clone())
        .with_thickness(selected)
        .scrollable(Action::MoveOverview)
        .selectable(Selectable::Track(track.id))
}
