use crate::app::Action;
use crate::key::Key;
use crate::popup::Popup;
use crate::time::{Instant, Signature, Tempo};
use crate::view::{Direction, OnClick, ToText as _, View};
use crate::{ToArcStr as _, UserInterface};
use arcstr::{ArcStr, literal};

const PLAY: ArcStr = literal!("\u{25B6}");
const PAUSE: ArcStr = literal!("\u{23F8}");

const PLAY_DESCRIPTION: ArcStr = literal!("play");
const PAUSE_DESCRIPTION: ArcStr = literal!("pause");

const KEY_DESCRIPTION: ArcStr = literal!("key");
const TIME_SIGNATURE_DESCRIPTION: ArcStr = literal!("time sig.");
const TEMPO_DESCRIPTION: ArcStr = literal!("tempo");

fn open_key_selector(instant: Instant, key: Key) -> OnClick {
    OnClick::from(Action::OpenPopup(Popup::KeySelector { instant, key }))
}

// TODO:
//  - window controls (opening instrument rack, piano roll, et.c)
//  -
//  - record, loop, metronome
//  - cursor fine positioning
//  - grid size
//  - master volume
/// The bar att the top of the app window.
pub fn bar<Ui: UserInterface>(
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

    let start_settings = View::balanced_stack::<Ui, _>(
        Direction::Right,
        [
            View::described_button(
                key.to_arc_str(),
                KEY_DESCRIPTION,
                open_key_selector(Instant::START, key),
            ),
            View::described_button(
                time_signature.to_arc_str(),
                TIME_SIGNATURE_DESCRIPTION,
                OnClick::default(),
            ),
            View::described_button(tempo.to_arc_str(), TEMPO_DESCRIPTION, OnClick::default()),
        ],
    );

    let left_side = View::spaced_stack::<Ui, _>(
        Direction::Right,
        vec![literal!("TODO").centred(), start_settings],
    );

    let right_side = literal!("TODO").centred();

    View::Stack {
        direction: Direction::Right,
        elements: vec![
            left_side.fill_remaining(),
            playback_button.quotated(Ui::PLAYBACK_BUTTON_WIDTH.get()),
            right_side.fill_remaining(),
        ],
    }
    .bordered()
    .titled(title)
    .with_thickness(true)
}
