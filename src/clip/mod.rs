pub mod audio;
mod content;

use ratatui::layout::Alignment;
use crate::clip::audio::Audio;
use crate::clip::content::Content;
use crate::id::Id;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use ratatui::style::Color;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::{Block, Borders};

const DEFAULT_AUDIO_COLOUR: Color = Color::Green;

#[derive(Clone, Debug)]
pub struct Clip {
    pub name: String,
    pub id: Id<Clip>,
    pub colour: Color,
    pub content: Content,
}

impl Clip {
    pub fn from_audio(name: String, audio: Audio) -> Clip {
        Clip {
            name,
            id: Id::new(),
            colour: DEFAULT_AUDIO_COLOUR,
            content: Content::Audio(audio),
        }
    }

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
                    .title(self.name.as_str()),
            )
    }
}
