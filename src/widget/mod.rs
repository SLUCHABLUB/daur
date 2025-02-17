pub mod bordered;
pub mod button;
pub mod heterogeneous;
pub mod homogenous;
mod injective;
pub mod macros;
pub mod multi;
pub mod single;
pub mod sized;
pub mod text;
pub mod to_widget;

use crate::app::action::Action;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::lock::Lock;
use crate::widget::macros::or_popup;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::{Block, WidgetRef as _};
use ratatui_explorer::{FileExplorer, Input};

/// Like [`Widget`](ratatui::widgets::Widget) but with mouse info.
#[must_use = "Widgets need to be rendered"]
pub trait Widget {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point);

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    );
}

// TODO: remove and add a custom clip-overview widget
impl<F: Fn(&mut Context)> Widget for Canvas<'_, F> {
    fn render(&self, area: Rectangle, buf: &mut Buffer, _: Point) {
        self.render_ref(area.to_rect(), buf);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {}
}

impl Widget for &Lock<FileExplorer> {
    fn render(&self, area: Rectangle, buf: &mut Buffer, _: Point) {
        self.read().widget().render_ref(area.to_rect(), buf);
    }

    fn click(&self, area: Rectangle, _: MouseButton, position: Point, actions: &mut Vec<Action>) {
        let area = area.to_rect();
        let position = position.to_position();
        let mut explorer = self.write();

        let inner_area = explorer
            .theme()
            .block()
            .unwrap_or(&Block::new())
            .inner(area);

        if inner_area.contains(position) {
            #[expect(
                clippy::arithmetic_side_effects,
                reason = "checked in the if statement"
            )]
            let index = usize::from(position.y - inner_area.y);

            if explorer.selected_idx() == index {
                or_popup!(explorer.handle(Input::Right), actions);
            } else if index < explorer.files().len() {
                explorer.set_selected_idx(index);
            } else {
                // clicked on padding
            }
        }
    }
}
