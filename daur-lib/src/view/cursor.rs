use crate::app::Action;
use crate::time::Instant;
use crate::ui::{Mapping, Offset};
use crate::view::{OnClick, View};

/// The musical cursor
pub fn cursor_window(cursor_position: Instant, mapping: Mapping, window_offset: Offset) -> View {
    let cursor_offset = mapping.x_offset(cursor_position);
    let cursor_offset = Offset::from(cursor_offset) + window_offset;
    if cursor_offset < Offset::ZERO {
        return View::Empty;
    }
    let offset = cursor_offset.rectify();

    let on_click = OnClick::new(move |_, position, actions| {
        let ui_offset = Offset::from(position.x) - window_offset;
        let instant = mapping.instant_on_grid(ui_offset.rectify());

        actions.send(Action::MoveCursor(instant));
    });

    View::CursorWindow { offset }.on_click(on_click)
}
