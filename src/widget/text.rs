use crate::app::action::Action;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::length::size::Size;
use crate::length::Length;
use crate::widget::sized::Sized;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Alignment;
use ratatui::widgets::Paragraph;
use std::cmp::max;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct Text {
    // TODO: use cow or arc
    pub string: String,
    pub centered: bool,
}

impl Text {
    pub fn left_aligned<S: Into<String>>(string: S) -> Text {
        Text {
            string: string.into(),
            centered: false,
        }
    }

    pub fn centered<S: Into<String>>(string: S) -> Text {
        Text {
            string: string.into(),
            centered: true,
        }
    }

    fn paragraph(&self) -> Paragraph {
        Paragraph::new(self.string.as_str()).alignment(if self.centered {
            Alignment::Center
        } else {
            Alignment::Left
        })
    }
}

impl Widget for Text {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        self.paragraph().render(area, buf, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        self.paragraph().click(area, button, position, actions);
    }
}

impl Sized for Text {
    fn size(&self) -> Size {
        let mut size = Size::ZERO;

        for line in self.string.lines() {
            size.width = max(size.width, Length::string_width(line));
        }

        size.height = Length::string_height(&self.string);

        size
    }
}
