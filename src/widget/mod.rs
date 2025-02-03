pub mod button;
pub mod homogenous_stack;
pub mod placeholder;
pub mod three_stack;
pub mod two_stack;

use crate::app::action::Action;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Position;
use ratatui::widgets::WidgetRef;

/// Like [`Widget`](ratatui::widgets::Widget) but with mouse info.
#[must_use = "Widgets need to be rendered"]
pub trait Widget {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position);

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    );
}

impl<T: WidgetRef> Widget for T {
    fn render(&self, area: Rect, buf: &mut Buffer, _: Position) {
        self.render_ref(area, buf);
    }

    fn click(&self, _: Rect, _: MouseButton, _: Position, _: &mut Vec<Action>) {}
}
