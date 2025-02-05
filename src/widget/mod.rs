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
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::{Block, Clear, Paragraph, WidgetRef};
use ratatui_explorer::FileExplorer;

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

impl<T: Widget> Widget for &T {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        (*self).render(area, buf, mouse_position);
    }

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
        (*self).click(area, button, position, action_queue);
    }
}

macro_rules! impl_widget_ref {
    ($widget:ty) => {
        impl Widget for $widget {
            fn render(&self, area: Rect, buf: &mut Buffer, _: Position) {
                self.render_ref(area, buf)
            }

            fn click(&self, _: Rect, _: MouseButton, _: Position, _: &mut Vec<Action>) {}
        }
    };
}

impl_widget_ref!(Block<'_>);
impl_widget_ref!(Clear);
impl_widget_ref!(Paragraph<'_>);

impl<F: Fn(&mut Context)> Widget for Canvas<'_, F> {
    fn render(&self, area: Rect, buf: &mut Buffer, _: Position) {
        self.render_ref(area, buf);
    }

    fn click(&self, _: Rect, _: MouseButton, _: Position, _: &mut Vec<Action>) {}
}

impl Widget for FileExplorer {
    fn render(&self, area: Rect, buf: &mut Buffer, _: Position) {
        self.widget().render_ref(area, buf);
    }

    fn click(&self, _: Rect, _: MouseButton, _: Position, _: &mut Vec<Action>) {}
}
