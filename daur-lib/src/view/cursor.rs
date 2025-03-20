use crate::app::Action;
use crate::time::Instant;
use crate::ui::{Mapping, Offset};
use crate::view::{OnClick, View};
use crossterm::event::MouseButton;

/// The musical cursor
pub fn cursor_window(cursor_position: Instant, mapping: Mapping, window_offset: Offset) -> View {
    let cursor_offset = mapping.offset(cursor_position);
    let cursor_offset = Offset::from(cursor_offset) + window_offset;
    let Some(offset) = cursor_offset.to_length() else {
        return View::EMPTY;
    };

    let on_click = OnClick::new(move |button, _, position, actions| {
        if button != MouseButton::Left {
            return;
        }

        let ui_offset = Offset::from(position.x) - window_offset;
        let instant = mapping.instant_on_grid(ui_offset.saturate());

        actions.send(Action::MoveCursor(instant));
    });

    View::CursorWindow { offset }.on_click(on_click)
}
