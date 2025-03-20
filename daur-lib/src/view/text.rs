use crate::view::{Alignment, View};
use arcstr::ArcStr;

// TODO: replace with extension trait on ArcStr
impl View {
    /// Constructs a top-left aligned text view
    pub fn top_left(string: ArcStr) -> Self {
        View::Text {
            string,
            alignment: Alignment::TopLeft,
        }
    }

    /// Constructs a top-right aligned text view
    pub fn top_right(string: ArcStr) -> Self {
        View::Text {
            string,
            alignment: Alignment::TopRight,
        }
    }

    /// Constructs a centered text view
    pub fn centred(string: ArcStr) -> Self {
        View::Text {
            string,
            alignment: Alignment::Centre,
        }
    }

    /// Constructs a bottom-right aligned text view
    pub fn bottom_right(string: ArcStr) -> Self {
        View::Text {
            string,
            alignment: Alignment::BottomRight,
        }
    }

    /* TODO: move to daur-tui
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
     */
}
