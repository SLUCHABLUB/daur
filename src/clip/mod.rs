pub mod content;
mod source;

use arcstr::ArcStr;
pub use source::ClipSource;

use crate::clip::content::Content;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::time::tempo::Tempo;
use crate::time::TimeSignature;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::{Block, Borders};

#[derive(Clone, Eq, PartialEq)]
pub struct Clip {
    pub name: ArcStr,
    pub colour: Color,
    pub content: Content,
}

impl Clip {
    pub fn period(
        &self,
        start: Instant,
        time_signature: &Changing<TimeSignature>,
        tempo: &Changing<Tempo>,
    ) -> Period {
        self.content.period(start, time_signature, tempo)
    }

    /// Returns the canvas for the clip overview.
    /// The viewport bounds have not yet been set.
    pub fn overview_canvas(&self, selected: bool) -> Canvas<impl Fn(&mut Context) + use<'_>> {
        let set = if selected { THICK } else { PLAIN };

        Canvas::default()
            .background_color(self.colour)
            .paint(|context| self.content.paint_overview(context))
            .block(
                Block::bordered()
                    .borders(Borders::TOP)
                    .title_alignment(Alignment::Center)
                    .border_set(set)
                    .title(self.name.as_str())
                    .style(Style::new().bg(self.colour)),
            )
    }

    pub fn to_source(&self, offset: usize) -> ClipSource {
        match &self.content {
            Content::Audio(audio) => ClipSource::Audio(audio.to_source(offset)),
            Content::Notes(_) => ClipSource::Notes,
        }
    }
}
