use crate::audio::Audio;
use crate::notes::Notes;
use crate::pitch::Pitch;
use crate::project::changing::Changing;
use crate::time::period::Period;
use crate::time::tempo::Tempo;
use crate::time::Instant;
use crate::time::TimeSignature;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Points};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Content {
    Audio(Audio),
    // TODO: linked audio file
    // TODO: linked clip
    Notes(Notes),
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
            Content::Notes(notes) => Period {
                start,
                duration: notes.duration(),
            },
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
            Content::Notes(notes) => {
                if let Some(range) = notes.pitch_range() {
                    let low = f64::from((*range.start() - Pitch::A440).semitones());
                    let high = f64::from((*range.start() - Pitch::A440).semitones());
                    [[0.0, notes.duration().whole_notes.to_float()], [low, high]]
                } else {
                    [[0.0; 2]; 2]
                }
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
            Content::Notes(notes) => notes.draw_overview(context),
        }
    }
}
