//! The UI of daur is based on views, based on the system used by the `ratatui` crate

mod alignment;
mod bordered;
mod button;
mod composition;
mod cursor;
mod direction;
mod feed;
mod has_size;
mod hoverable;
mod macros;
mod ruler;
mod solid;
mod text;

pub mod heterogeneous;
pub mod homogenous;
pub mod multi;
mod reference;
pub mod single;
mod size_informed;

pub use alignment::Alignment;
pub use bordered::Bordered;
pub use button::{Button, OnClick};
pub use composition::Composition;
pub use cursor::CursorWindow;
pub use direction::Direction;
pub use feed::Feed;
pub use has_size::HasSize;
pub use hoverable::Hoverable;
pub(crate) use macros::{or_popup, popup_error};
pub use reference::Ref;
pub use ruler::Ruler;
pub use size_informed::SizeInformed;
pub use solid::Solid;
pub use text::Text;

use crate::app::Action;
use crate::lock::Lock;
use crate::ui::{Point, Rectangle};
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::widgets::{Block, Widget};
use ratatui_explorer::{FileExplorer, Input};

/// A UI element.
#[must_use]
pub trait View {
    /// Render the view in the given area in `buffer`
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point);

    /// Click the view
    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    );
}

impl<T: View> View for Option<T> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        if let Some(view) = self {
            view.render(area, buffer, mouse_position);
        }
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        if let Some(view) = self {
            view.click(area, button, position, actions);
        }
    }
}

impl<T: View, E: View> View for Result<T, E> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        match self {
            Ok(ok) => ok.render(area, buffer, mouse_position),
            Err(err) => err.render(area, buffer, mouse_position),
        }
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        match self {
            Ok(ok) => ok.click(area, button, position, actions),
            Err(err) => err.click(area, button, position, actions),
        }
    }
}

impl View for Lock<FileExplorer> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, _: Point) {
        Widget::render(self.read().widget(), area.to_rect(), buffer);
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
