use crate::app::Action;
use crate::metre::Instant;
use crate::notes::Key;
use crate::popup::Popup;
use crate::project::Settings;
use crate::view::{Axis, OnClick, ToText as _, View};
use crate::{ToArcStr as _, UserInterface};
use arcstr::{ArcStr, literal};

const PLAY: ArcStr = literal!("\u{25B6}");
const PAUSE: ArcStr = literal!("\u{23F8}");
const EDIT: ArcStr = literal!("\u{270E}");
const SELECT: ArcStr = literal!("\u{1FBB0}");
const PIANO: ArcStr = literal!("\u{1F3B9}");

const PLAY_DESCRIPTION: ArcStr = literal!("play");
const PAUSE_DESCRIPTION: ArcStr = literal!("pause");
const EDIT_DESCRIPTION: ArcStr = literal!("edit mode");
const SELECT_DESCRIPTION: ArcStr = literal!("select mode");
const PIANO_DESCRIPTION: ArcStr = literal!("piano roll");

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
    settings: &Settings,
    playing: bool,
    edit_mode: bool,
) -> View {
    let playback_button = if playing {
        View::described_button(PAUSE, PAUSE_DESCRIPTION, OnClick::from(Action::Pause))
    } else {
        View::described_button(PLAY, PLAY_DESCRIPTION, OnClick::from(Action::Play))
    };
    let edit_button = if edit_mode {
        View::described_button(
            SELECT,
            SELECT_DESCRIPTION,
            OnClick::from(Action::ExitEditMode),
        )
    } else {
        View::described_button(EDIT, EDIT_DESCRIPTION, OnClick::from(Action::EnterEditMode))
    };

    // TODO: show current settings?
    let project_settings = View::balanced_stack::<Ui, _>(
        Axis::X,
        [
            View::described_button(
                settings.key.start.to_arc_str(),
                KEY_DESCRIPTION,
                open_key_selector(Instant::START, settings.key.start),
            ),
            View::described_button(
                settings.time_signature.start.to_arc_str(),
                TIME_SIGNATURE_DESCRIPTION,
                OnClick::default(),
            ),
            View::described_button(
                settings.tempo.start.to_arc_str(),
                TEMPO_DESCRIPTION,
                OnClick::default(),
            ),
        ],
    );

    let toggles = View::spaced_stack::<Ui, _>(
        Axis::X,
        [
            edit_button,
            View::described_button(
                PIANO,
                PIANO_DESCRIPTION,
                OnClick::from(Action::TogglePianoRoll),
            ),
        ],
    );

    let left_side =
        View::spaced_stack::<Ui, _>(Axis::X, [literal!("TODO").centred(), project_settings]);

    let right_side = View::spaced_stack::<Ui, _>(Axis::X, [literal!("TODO").centred(), toggles]);

    View::x_stack([
        left_side.fill_remaining(),
        playback_button.quotated(Ui::PLAYBACK_BUTTON_WIDTH.get()),
        right_side.fill_remaining(),
    ])
    .bordered()
    .titled(title)
    .with_thickness(true)
}
