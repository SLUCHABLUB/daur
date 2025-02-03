mod audio;
mod content;

use crate::clip::content::Content;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use ratatui::style::Color;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::{Block, Borders};

#[derive(Clone, Debug)]
pub struct Clip {
    pub name: String,
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
                    .border_set(set)
                    .title(self.name.as_str()),
            )
    }
}
