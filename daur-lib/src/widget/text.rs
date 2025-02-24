use crate::app::Action;
use crate::ui::{Length, NonZeroLength, Point, Rectangle, Size};
use crate::widget::has_size::HasSize;
use crate::widget::Widget;
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Alignment;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, WidgetRef as _};
use saturating_cast::SaturatingCast as _;
use std::cmp::max;

/// Some text
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Text {
    /// The text
    pub string: ArcStr,
    /// Whether the text is centred or not (top-left aligned)
    pub centered: bool,
}

impl Text {
    /// Constructs a top-left aligned text widget
    #[must_use]
    pub fn left_aligned(string: ArcStr) -> Text {
        Text {
            string,
            centered: false,
        }
    }

    /// Constructs a centered text widget
    #[must_use]
    pub fn centred(string: ArcStr) -> Text {
        Text {
            string,
            centered: true,
        }
    }

    fn paragraph(&self, height: Length) -> Paragraph {
        #![expect(
            clippy::indexing_slicing,
            reason = "by dividing `line_count` by 2, the index will be inside `lines`"
        )]

        if self.centered {
            let line_count = (height / NonZeroLength::CHAR_HEIGHT)
                .round()
                .saturating_cast();

            if line_count == 0 {
                return Paragraph::new("");
            }

            let mut lines = vec![Line::raw(""); line_count];

            #[expect(clippy::integer_division, reason = "favour top by rounding down")]
            let halfway = line_count.saturating_sub(1) / 2;

            lines[halfway] = Line::raw(self.string.as_str());

            Paragraph::new(lines).alignment(Alignment::Center)
        } else {
            Paragraph::new(self.string.as_str())
        }
    }
}

impl Widget for Text {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, _: Point) {
        self.paragraph(area.height)
            .render_ref(area.to_rect(), buffer);
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
