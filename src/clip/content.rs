use crate::audio::Audio;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::time::tempo::Tempo;
use crate::time::TimeSignature;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Points};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Content {
    Audio(Audio),
    // TODO: linked audio file
    // TODO: linked clip
    // TODO: midi
    // TODO: drums
}

impl Content {
    pub fn period(
        &self,
        start: Instant,
        time_signature: &Changing<TimeSignature>,
        tempo: &Changing<Tempo>,
    ) -> Period {
        match self {
            Content::Audio(audio) => audio.period(start, time_signature, tempo),
        }
    }

    /// The full viewport of the overview.
    /// This is used if the whole clip overview is visible.
    pub fn full_overview_viewport(&self) -> [[f64; 2]; 2] {
        match self {
            Content::Audio(audio) => {
                #[expect(clippy::cast_precision_loss, reason = "We are just drawing")]
                let viewport_end = audio.sample_count() as f64;
                [[0.0, viewport_end], [-1.0, 1.0]]
            }
        }
    }

    pub fn paint_overview(&self, context: &mut Context) {
        match self {
            Content::Audio(audio) => {
                #[expect(clippy::cast_precision_loss, reason = "We are just drawing")]
                let points: Vec<(f64, f64)> = audio
                    .mono_samples()
                    .enumerate()
                    .map(|(index, sample)| (index as f64, sample))
                    .collect();

                context.draw(&Points {
                    coords: &points,
                    color: Color::Reset,
                });
            }
        }
    }
}
