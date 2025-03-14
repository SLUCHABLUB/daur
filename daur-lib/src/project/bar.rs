use crate::app::Action;
use crate::key::Key;
use crate::popup::Popup;
use crate::time::{Signature, Tempo};
use crate::widget::heterogeneous::{ThreeStack, TwoStack};
use crate::widget::{Bordered, Button, Hoverable, OnClick, Text, ToWidget};
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

fn select_key(key: Key) -> OnClick<'static> {
    OnClick::from(Action::OpenPopup(Popup::key_selector(key)))
}

/// The bar att the top of the app window.
#[derive(Debug)]
pub struct Bar {
    /// The title of the project.
    pub title: ArcStr,
    /// The default tempo of the project.
    pub tempo: Tempo,
    /// The default time signature of the project.
    pub time_signature: Signature,
    /// The default key of the project.
    pub key: Key,
    /// Whether the audio is playing.
    pub playing: bool,
}

// TODO: window controls (opening instrument rack, piano roll, et.c)

// TODO: record, loop, metronome
// TODO: cursor fine positioning
// TODO: grid size
// TODO: master volume
impl ToWidget for Bar {
    type Widget<'widget> = Bordered<
        ThreeStack<
            TwoStack<
                Text,
                ThreeStack<
                    Button<'static, Hoverable<Bordered<Text>>>,
                    Button<'static, Hoverable<Bordered<Text>>>,
                    Button<'static, Hoverable<Bordered<Text>>>,
                >,
            >,
            Button<'static, Hoverable<Bordered<Text>>>,
            Text,
        >,
    >;

    fn to_widget(&self) -> Self::Widget<'_> {
        let playback_button = if self.playing {
            Button::described(PAUSE, PAUSE_DESCRIPTION, OnClick::from(Action::Pause))
        } else {
            Button::described(PLAY, PLAY_DESCRIPTION, OnClick::from(Action::Play))
        };

        let fallbacks = ThreeStack::equisized_horizontal((
            Button::described(self.key.to_arc_str(), KEY_DESCRIPTION, select_key(self.key)),
            Button::described(
                self.time_signature.to_arc_str(),
                TIME_SIGNATURE_DESCRIPTION,
                OnClick::default(),
            ),
            Button::described(
                self.tempo.to_arc_str(),
                TEMPO_DESCRIPTION,
                OnClick::default(),
            ),
        ));

        let left_side =
            TwoStack::equisized_horizontal((Text::centred(literal!("TODO")), fallbacks))
                .flex(Flex::SpaceBetween);

        Bordered::titled(
            ArcStr::clone(&self.title),
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
        .thickness(true)
    }
}
