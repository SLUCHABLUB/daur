use crate::app::Action;
use crate::key::Key;
use crate::popup::Popup;
use crate::project::Project;
use crate::widget::heterogeneous::{ThreeStack, TwoStack};
use crate::widget::{Bordered, Button, Text, Widget};
use crate::ToArcStr as _;
use arcstr::{literal, ArcStr};
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

        let fallbacks = ThreeStack::equisized_horizontal((
            Button::described(
                self.key.start.to_arc_str(),
                KEY_DESCRIPTION,
                select_key(self.key.start),
            ),
            Button::described(
                self.time_signature.start.to_arc_str(),
                TIME_SIGNATURE_DESCRIPTION,
                Action::None,
            ),
            Button::described(
                self.tempo.start.to_arc_str(),
                TEMPO_DESCRIPTION,
                Action::None,
            ),
        ));

        let left_side =
            TwoStack::equisized_horizontal((Text::centred(literal!("TODO")), fallbacks))
                .flex(Flex::SpaceBetween);

        Bordered::thick(
            self.title(),
            ThreeStack::horizontal(
                (left_side, playback_button, Text::centred(literal!("TODO"))),
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
