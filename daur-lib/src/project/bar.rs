use crate::app::Action;
use crate::key::Key;
use crate::popup::Popup;
use crate::project::Project;
use crate::widget::heterogeneous::{ThreeStack, TwoStack};
use crate::widget::{Bordered, Button, Text, Widget};
use arcstr::{format, literal, ArcStr};
use ratatui::layout::{Constraint, Flex};

const PLAY: ArcStr = literal!("\u{25B6}");
const PAUSE: ArcStr = literal!("\u{23F8}");

const PLAY_DESCRIPTION: ArcStr = literal!("play");
const PAUSE_DESCRIPTION: ArcStr = literal!("pause");

const KEY_DESCRIPTION: ArcStr = literal!("key");
const TIME_SIGNATURE_DESCRIPTION: ArcStr = literal!("time sig.");
const TEMPO_DESCRIPTION: ArcStr = literal!("tempo");

pub fn select_key(key: Key) -> Action {
    Action::OpenPopup(Popup::key_selector(key))
}

impl Project {
    pub fn title(&self) -> ArcStr {
        ArcStr::clone(&self.title)
    }

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
                    format!("{}", self.key.start),
                    KEY_DESCRIPTION,
                    select_key(self.key.start),
                ),
                Button::described(
                    format!("{}", self.time_signature.start),
                    TIME_SIGNATURE_DESCRIPTION,
                    Action::None,
                ),
                Button::described(
                    format!("{}", self.tempo.start),
                    TEMPO_DESCRIPTION,
                    Action::None,
                ),
            ),
            [Constraint::Fill(1); 3],
        );

        let left_side = TwoStack::horizontal(
            (Text::centered(literal!("TODO")), fallbacks),
            [Constraint::Fill(1); 2],
        )
        .flex(Flex::SpaceBetween);

        Bordered::thick(
            self.title(),
            ThreeStack::horizontal(
                (left_side, playback_button, Text::centered(literal!("TODO"))),
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
