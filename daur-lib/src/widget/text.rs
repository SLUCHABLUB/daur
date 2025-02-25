use crate::app::Action;
use crate::ui::{Length, NonZeroLength, Point, Rectangle, Size};
use crate::widget::has_size::HasSize;
use crate::widget::{Alignment, Widget};
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::{layout, widgets};
use saturating_cast::SaturatingCast as _;
use std::cmp::max;

/// Some text
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Text {
    /// The text
    pub string: ArcStr,
    /// How the text should be aligned
    pub alignment: Alignment,
}

impl Text {
    /// Constructs a top-left aligned text widget
    #[must_use]
    pub fn top_left(string: ArcStr) -> Text {
        Text {
            string,
            alignment: Alignment::TopLeft,
        }
    }

    /// Constructs a top-right aligned text widget
    #[must_use]
    pub fn top_right(string: ArcStr) -> Text {
        Text {
            string,
            alignment: Alignment::TopRight,
        }
    }

    /// Constructs a centered text widget
    #[must_use]
    pub fn centred(string: ArcStr) -> Text {
        Text {
            string,
            alignment: Alignment::Centre,
        }
    }

    /// Constructs a bottom-right aligned text widget
    #[must_use]
    pub fn bottom_right(string: ArcStr) -> Text {
        Text {
            string,
            alignment: Alignment::BottomRight,
        }
    }

    fn paragraph(&self, height: Length) -> Paragraph {
        let alignment = match self.alignment {
            Alignment::TopLeft | Alignment::Left | Alignment::BottomLeft => layout::Alignment::Left,
            Alignment::Top | Alignment::Centre | Alignment::Bottom => layout::Alignment::Center,
            Alignment::TopRight | Alignment::Right | Alignment::BottomRight => {
                layout::Alignment::Right
            }
        };

        let full_line_count: usize = (height / NonZeroLength::CHAR_HEIGHT)
            .round()
            .saturating_cast();

        let Some(max_padding) = full_line_count.checked_sub(1) else {
            return Paragraph::new("");
        };

        #[expect(clippy::integer_division, reason = "favour top by rounding down")]
        let mut padding = match self.alignment {
            Alignment::TopLeft | Alignment::Top | Alignment::TopRight => 0,
            Alignment::Left | Alignment::Centre | Alignment::Right => max_padding / 2,
            Alignment::BottomLeft | Alignment::Bottom | Alignment::BottomRight => max_padding,
        };

        padding = padding.saturating_sub(self.string.lines().count());

        let mut lines = vec![Line::raw(""); padding];
        lines.extend(self.string.lines().map(Line::raw));

        Paragraph::new(lines).alignment(alignment)
    }
}

impl Widget for Text {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, _: Point) {
        widgets::Widget::render(self.paragraph(area.size.height), area.to_rect(), buffer);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {}
}

impl HasSize for Text {
    fn size(&self) -> Size {
        let mut size = Size::ZERO;

        for line in self.string.lines() {
            size.width = max(size.width, Length::string_width(line));
        }

        size.height = Length::string_height(&self.string);

        size
    }
}
