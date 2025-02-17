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
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, WidgetRef as _};
use saturating_cast::SaturatingCast as _;
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

    fn paragraph(&self, height: Length) -> Paragraph {
        #![expect(
            clippy::indexing_slicing,
            reason = "by dividing `line_count` by 2, the index will be inside `lines`"
        )]

        if self.centered {
            let line_count = (height / Length::CHAR_HEIGHT).round().saturating_cast();
            let mut lines = vec![Line::raw(""); line_count];

            #[expect(clippy::integer_division, reason = "favour top by rounding down")]
            let halfway = line_count / 2;

            lines[halfway] = Line::raw(self.string.as_str());

            Paragraph::new(lines).alignment(Alignment::Center)
        } else {
            Paragraph::new(self.string.as_str())
        }
    }
}

impl Widget for Text {
    fn render(&self, area: Rectangle, buf: &mut Buffer, _: Point) {
        self.paragraph(area.height).render_ref(area.to_rect(), buf);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {}
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
