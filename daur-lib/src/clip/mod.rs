mod content;
mod source;

pub use content::ClipContent;
pub use source::ClipSource;

use crate::time::{Instant, Mapping, Period};
use arcstr::ArcStr;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::{Block, Borders};

/// A clip inside a [`Track`]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Clip {
    /// The name of the clip
    pub name: ArcStr,
    /// The colour of the clip
    pub colour: Color,
    /// The content of the clip
    pub content: ClipContent,
}

impl Clip {
    /// The [`Period`] of the clip
    #[must_use]
    pub fn period(&self, start: Instant, mapping: &Mapping) -> Period {
        self.content.period(start, mapping)
    }

    /// Returns the canvas for the clip overview.
    /// The viewport bounds have not yet been set.
    pub(crate) fn overview_canvas(
        &self,
        selected: bool,
    ) -> Canvas<impl Fn(&mut Context) + use<'_>> {
        let set = if selected { THICK } else { PLAIN };

        Canvas::default()
            .background_color(self.colour)
            .paint(|context| self.content.paint_overview(context))
            .block(
                Block::bordered()
                    .borders(Borders::TOP)
                    .title_alignment(Alignment::Right)
                    .border_set(set)
                    .title(self.name.as_str())
                    .style(Style::new().bg(self.colour)),
            )
    }

    /// Returns a [`Source`](rodio::source::Source) for the clip
    pub fn to_source(&self, offset: usize) -> ClipSource {
        match &self.content {
            ClipContent::Audio(audio) => ClipSource::Audio(audio.to_source(offset)),
            ClipContent::Notes(_) => ClipSource::Notes,
        }
    }
}
