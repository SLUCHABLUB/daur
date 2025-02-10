pub mod change;
pub mod changing;
mod source;

use crate::app::action::Action;
use crate::app::ruler::Ruler;
use crate::app::settings::OverviewSettings;
use crate::clip::Clip;
use crate::columns::ScreenLength;
use crate::id::Id;
use crate::key::Key;
use crate::project::changing::Changing;
use crate::project::source::ProjectSource;
use crate::time::instant::Instant;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use crate::track::Track;
use crate::widget::button::Button;
use crate::widget::homogenous_stack::HomogenousStack;
use crate::widget::three_stack::ThreeStack;
use crate::widget::two_stack::TwoStack;
use crate::widget::Widget;
use ratatui::layout::Flex;
use ratatui::prelude::Constraint;
use ratatui::symbols::border::THICK;
use ratatui::widgets::{Block, Clear, Paragraph};
use saturating_cast::SaturatingCast;
use std::borrow::Cow;

const PLAY: &str = "\u{25B6}";
const PAUSE: &str = "\u{23F8}";

const PLAY_DESCRIPTION: &str = "play";
const PAUSE_DESCRIPTION: &str = "pause";

#[derive(Clone, Debug, Default)]
pub struct Project {
    pub title: String,

    pub key: Changing<Key>,
    pub time_signature: Changing<TimeSignature>,
    // TODO: continuous change
    pub tempo: Changing<Tempo>,

    pub tracks: Vec<Track>,
}

impl Project {
    // TODO: window controls (opening instrument rack, piano roll, et.c)
    // TODO: key, time sig., tempo

    // TODO: record, loop, metronome
    // TODO: cursor fine positioning
    // TODO: grid size
    // TODO: master volume
    pub fn bar(&self, playing: bool) -> impl Widget + use<'_> {
        let playback_button = if playing {
            Button::new(PAUSE, Action::Pause)
                .description(PAUSE_DESCRIPTION)
                .bordered()
        } else {
            Button::new(PLAY, Action::Play)
                .description(PLAY_DESCRIPTION)
                .bordered()
        };

        let block = Block::bordered()
            .border_set(THICK)
            .title(self.title.as_str());

        ThreeStack::horizontal(
            (
                Paragraph::new("TODO").centered(),
                playback_button,
                Paragraph::new("TODO").centered(),
            ),
            [
                Constraint::Fill(1),
                Constraint::Length(7),
                Constraint::Fill(1),
            ],
        )
        .block(block)
        .flex(Flex::Center)
    }

    pub fn workspace(
        &self,
        track_settings_size: ScreenLength,
        overview_settings: OverviewSettings,
        selected_track: Id<Track>,
        selected_clip: Id<Clip>,
        cursor: Instant,
    ) -> impl Widget + use<'_> {
        let track_count = self.tracks.len().saturating_cast();

        let horizontal_constraints = [
            Constraint::Length(track_settings_size.get()),
            Constraint::Fill(1),
        ];
        let vertical_constraints = [
            Constraint::Max(2),
            Constraint::Fill(track_count),
            Constraint::Fill(1),
        ];

        let ruler = Ruler {
            time_signature: &self.time_signature,
            overview_settings,
        };
        let ruler_row = TwoStack::horizontal((Clear, ruler), horizontal_constraints);

        let tracks = HomogenousStack::equidistant_vertical(self.tracks.iter().map(move |track| {
            let selected = track.id == selected_track;
            TwoStack::horizontal(
                (
                    track.settings(selected),
                    track.overview(
                        selected_clip,
                        &self.time_signature,
                        &self.tempo,
                        overview_settings,
                        cursor,
                    ),
                ),
                horizontal_constraints,
            )
        }));

        let add_track_button = Button::new(Cow::Borrowed("+"), Action::AddTrack)
            .description(Cow::Borrowed("add track"))
            .bordered();

        let add_track_row = TwoStack::horizontal((add_track_button, Clear), horizontal_constraints);

        ThreeStack::vertical((ruler_row, tracks, add_track_row), vertical_constraints)
    }

    pub fn to_source(&self, sample_rate: u32, cursor: Instant) -> ProjectSource {
        let offset = cursor.to_sample(&self.time_signature, &self.tempo, sample_rate);
        ProjectSource {
            sample_rate,
            tracks: self
                .tracks
                .iter()
                .map(|track| {
                    track.to_source(&self.time_signature, &self.tempo, sample_rate, offset)
                })
                .collect(),
        }
    }
}
