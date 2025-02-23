use crate::audio::Audio;
use crate::notes::Notes;
use crate::pitch::Pitch;
use crate::time::{Instant, Mapping, Period};
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Points};

/// The content of a [`Clip`](crate::Clip)
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ClipContent {
    /// An audio clip
    Audio(Audio),
    // TODO: linked audio file
    // TODO: linked clip
    /// A notes clip
    Notes(Notes),
    // TODO: drums
}

impl ClipContent {
    /// Returns the period of the content
    #[must_use]
    pub fn period(&self, start: Instant, mapping: &Mapping) -> Period {
        match self {
            ClipContent::Audio(audio) => audio.period(start, mapping),
            ClipContent::Notes(notes) => Period {
                start,
                duration: notes.duration(),
            },
        }
    }

    /// The full viewport of the overview.
    /// This is used if the whole clip overview is visible.
    pub(crate) fn full_overview_viewport(&self) -> [[f64; 2]; 2] {
        match self {
            ClipContent::Audio(audio) => {
                #[expect(clippy::cast_precision_loss, reason = "We are just drawing")]
                let viewport_end = audio.sample_count() as f64;
                [[0.0, viewport_end], [-1.0, 1.0]]
            }
            ClipContent::Notes(notes) => {
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

    pub(super) fn paint_overview(&self, context: &mut Context) {
        match self {
            ClipContent::Audio(audio) => {
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
            ClipContent::Notes(notes) => notes.draw_overview(context),
        }
    }
}
