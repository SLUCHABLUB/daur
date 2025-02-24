use crate::app::Action;
use crate::ui::{NonZeroLength, Point, Rectangle};
use crate::widget::{Text, Widget};
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use itertools::Itertools as _;
use ratatui::buffer::Buffer;
use ratatui::symbols::line::VERTICAL;
use saturating_cast::SaturatingCast as _;
use std::iter::repeat_n;

pub struct Cursor;

impl Widget for Cursor {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let height = (area.height / NonZeroLength::CHAR_HEIGHT)
            .round()
            .saturating_cast();

        Text::top_left(ArcStr::from(repeat_n(VERTICAL, height).join("\n"))).render(
            area,
            buffer,
            mouse_position,
        );
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {}
}
