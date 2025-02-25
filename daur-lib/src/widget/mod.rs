//! The UI of daur is based on widgets, based on the system used by the `ratatui` crate

mod bordered;
mod button;
mod feed;
mod has_size;
// TODO: reexport
pub(crate) mod injective;
mod macros;
mod solid;
mod text;
mod to_widget;

mod alignment;
mod direction;
pub mod heterogeneous;
pub mod homogenous;
pub mod multi;
pub mod single;

pub use alignment::Alignment;
pub use bordered::Bordered;
pub use button::Button;
pub use direction::Direction;
pub use feed::{feed, Feed};
pub use has_size::HasSize;
pub(crate) use macros::{or_popup, popup_error};
pub use solid::Solid;
pub use text::Text;
pub use to_widget::ToWidget;

use crate::app::Action;
use crate::lock::Lock;
use crate::ui::{Point, Rectangle};
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::widgets;
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::Block;
use ratatui_explorer::{FileExplorer, Input};

/// Like [`Widget`](ratatui::widgets::Widget) but with mouse info.
#[must_use = "Widgets don't render themselves"]
pub trait Widget {
    /// Render the widget in the given area in `buffer`
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point);

    /// Click the widget
    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    );
}

impl<T: Widget> Widget for Option<T> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        if let Some(widget) = self {
            widget.render(area, buffer, mouse_position);
        }
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        if let Some(widget) = self {
            widget.click(area, button, position, actions);
        }
    }
}

// TODO: remove and add a custom clip-overview widget
impl<F: Fn(&mut Context)> Widget for Canvas<'_, F> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, _: Point) {
        widgets::Widget::render(self, area.to_rect(), buffer);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {}
}

impl Widget for &Lock<FileExplorer> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, _: Point) {
        widgets::Widget::render(self.read().widget(), area.to_rect(), buffer);
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
