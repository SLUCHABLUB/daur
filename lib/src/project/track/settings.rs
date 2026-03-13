//! File for the [`settings`] function.

use crate::Selectable;
use crate::View;
use crate::app::Action;
use crate::app::Workspace;
use crate::project::Track;
use crate::view::Axis;
use crate::view::OnClick;
use crate::view::ToText as _;
use arcstr::literal;

/// Returns the track settings.
pub(crate) fn settings(track: &Track, selected: bool) -> View {
    // TODO: Make this some kind of reactive-direction stack.

    View::balanced_stack(
        Axis::Y,
        [
            literal!("TODO: volume here").centred().bordered(),
            literal!("TODO: pan, mute & solo here").centred().bordered(),
            View::standard_button(
                literal!("instrument"),
                OnClick::from(Action::ToggleWorkspace(Workspace::Plugins)),
            ),
        ],
    )
    .bordered_with_title_and_thickness(track.name.clone(), selected)
    .scrollable(Action::MoveOverview)
    .selectable(Selectable::Track(track.id))
}
