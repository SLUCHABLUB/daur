use crate::app::Action;
use crate::key::Key;
use crate::popup::Popup;
use crate::time::{Signature, Tempo};
use crate::ui::Length;
use crate::view::{Direction, OnClick, View};
use crate::ToArcStr as _;
use arcstr::{literal, ArcStr};

const PLAY: ArcStr = literal!("\u{25B6}");
const PAUSE: ArcStr = literal!("\u{23F8}");

const PLAY_DESCRIPTION: ArcStr = literal!("play");
const PAUSE_DESCRIPTION: ArcStr = literal!("pause");

const KEY_DESCRIPTION: ArcStr = literal!("key");
const TIME_SIGNATURE_DESCRIPTION: ArcStr = literal!("time sig.");
const TEMPO_DESCRIPTION: ArcStr = literal!("tempo");

fn select_key(key: Key) -> OnClick {
    OnClick::from(Action::OpenPopup(Popup::key_selector(key)))
}

// TODO:
//  - window controls (opening instrument rack, piano roll, et.c)
//  -
//  - record, loop, metronome
//  - cursor fine positioning
//  - grid size
//  - master volume
/// The bar att the top of the app window.
pub fn bar(
    title: ArcStr,
    tempo: Tempo,
    time_signature: Signature,
    key: Key,
    playing: bool,
) -> View {
    let playback_button = if playing {
        View::described_button(PAUSE, PAUSE_DESCRIPTION, OnClick::from(Action::Pause))
    } else {
        View::described_button(PLAY, PLAY_DESCRIPTION, OnClick::from(Action::Play))
    };

    let fallbacks = View::balanced_stack(
        Direction::Right,
        [
            View::described_button(key.to_arc_str(), KEY_DESCRIPTION, select_key(key)),
            View::described_button(
                time_signature.to_arc_str(),
                TIME_SIGNATURE_DESCRIPTION,
                OnClick::default(),
            ),
            View::described_button(tempo.to_arc_str(), TEMPO_DESCRIPTION, OnClick::default()),
        ],
    );

    let left_side = View::spaced_stack(
        Direction::Right,
        vec![View::centred(literal!("TODO")), fallbacks],
    );

    let right_side = View::centred(literal!("TODO"));

    View::Stack {
        direction: Direction::Right,
        elements: vec![
            left_side.fill_remaining(),
            playback_button.quotated(Length::PLAYBACK_BUTTON_WIDTH),
            right_side.fill_remaining(),
        ],
    }
    .titled(title)
    .with_thickness(true)
}
