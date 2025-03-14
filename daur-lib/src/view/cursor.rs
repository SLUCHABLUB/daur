use crate::app::Action;
use crate::time::Instant;
use crate::ui::{Length, Mapping, NonZeroLength, Offset, Point, Rectangle, Size};
use crate::view::{Text, View};
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use itertools::Itertools as _;
use ratatui::buffer::Buffer;
use ratatui::symbols::line::VERTICAL;
use saturating_cast::SaturatingCast as _;
use std::iter::repeat_n;

/// The musical cursor
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CursorWindow {
    /// The ui-mapping used for the cursor window
    pub mapping: Mapping,
    /// How far the window is scrolled
    pub offset: Offset,
    /// The position of the cursor
    pub instant: Instant,
}

impl View for CursorWindow {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let cursor_offset = self.mapping.offset(self.instant);
        let cursor_offset = Offset::from(cursor_offset) + self.offset;
        let Some(cursor_offset) = cursor_offset.to_length() else {
            return;
        };
        if area.size.width <= cursor_offset {
            return;
        }

        let cursor_area = Rectangle {
            position: Point {
                x: cursor_offset + area.position.x,
                y: area.position.y,
            },
            size: Size {
                width: Length::CURSOR_WIDTH,
                height: area.size.height,
            },
        };

        let line_count = (area.size.height / NonZeroLength::CHAR_HEIGHT)
            .round()
            .saturating_cast();

        Text::top_left(ArcStr::from(repeat_n(VERTICAL, line_count).join("\n"))).render(
            cursor_area,
            buffer,
            mouse_position,
        );
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        if button != MouseButton::Left {
            return;
        }

        let ui_offset = Offset::from(position.x - area.position.x) - self.offset;
        let instant = self.mapping.instant_on_grid(ui_offset.saturate());

        actions.push(Action::MoveCursor(instant));
    }
}
