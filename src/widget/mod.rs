pub mod block;
pub mod button;
pub mod heterogeneous_stack;
pub mod homogenous_stack;
pub mod macros;
pub mod multi_selector;
pub mod placeholder;
pub mod single_selector;
pub mod sized;
pub mod to_widget;

use crate::app::action::Action;
use crate::lock::Lock;
use crate::widget::macros::or_popup;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Position;
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::{Block, Clear, Paragraph, WidgetRef};
use ratatui_explorer::{FileExplorer, Input};

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

impl Widget for Lock<FileExplorer> {
    fn render(&self, area: Rect, buf: &mut Buffer, _: Position) {
        self.read().widget().render_ref(area, buf);
    }

    fn click(
        &self,
        area: Rect,
        _: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
        let mut explorer = self.write();

        let inner_area = explorer
            .theme()
            .block()
            .unwrap_or(&Block::new())
            .inner(area);

        if inner_area.contains(position) {
            let index = usize::from(position.y - inner_area.y);

            if explorer.selected_idx() == index {
                or_popup!(explorer.handle(Input::Right), action_queue);
            } else if index < explorer.files().len() {
                explorer.set_selected_idx(index);
            }
        }
    }
}
