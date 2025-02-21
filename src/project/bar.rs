use crate::app::Action;
use crate::key::Key;
use crate::popup::Popup;
use crate::project::Project;
use crate::widget::bordered::Bordered;
use crate::widget::button::Button;
use crate::widget::heterogeneous::{ThreeStack, TwoStack};
use crate::widget::text::Text;
use crate::widget::Widget;
use ratatui::layout::{Constraint, Flex};
use std::sync::Arc;

const PLAY: &str = "\u{25B6}";
const PAUSE: &str = "\u{23F8}";

const PLAY_DESCRIPTION: &str = "play";
const PAUSE_DESCRIPTION: &str = "pause";

const KEY_DESCRIPTION: &str = "key";
const TIME_SIGNATURE_DESCRIPTION: &str = "time sig.";
const TEMPO_DESCRIPTION: &str = "tempo";

pub fn select_key(key: Key) -> Action {
    Action::OpenPopup(Popup::key_selector(key))
}

impl Project {
    // TODO: window controls (opening instrument rack, piano roll, et.c)

    // TODO: record, loop, metronome
    // TODO: cursor fine positioning
    // TODO: grid size
    // TODO: master volume
    pub fn bar(&self, playing: bool) -> impl Widget {
        let playback_button = if playing {
            Button::described(PAUSE, PAUSE_DESCRIPTION, Action::Pause)
        } else {
            Button::described(PLAY, PLAY_DESCRIPTION, Action::Play)
        };

        let fallbacks = ThreeStack::horizontal(
            (
                Button::described(
                    self.key.start.to_string(),
                    KEY_DESCRIPTION,
                    select_key(self.key.start),
                ),
                Button::described(
                    self.time_signature.start.to_string(),
                    TIME_SIGNATURE_DESCRIPTION,
                    Action::None,
                ),
                Button::described(
                    self.tempo.start.to_string(),
                    TEMPO_DESCRIPTION,
                    Action::None,
                ),
            ),
            [Constraint::Fill(1); 3],
        );

        let left_side = TwoStack::horizontal(
            (Text::centered("TODO"), fallbacks),
            [Constraint::Fill(1); 2],
        )
        .flex(Flex::SpaceBetween);

        Bordered::thick(
            Arc::clone(&self.title),
            ThreeStack::horizontal(
                (left_side, playback_button, Text::centered("TODO")),
                [
                    Constraint::Fill(1),
                    Constraint::Length(7),
                    Constraint::Fill(1),
                ],
            )
            .flex(Flex::Center),
        )
    }
}
