use crate::app::Action;
use crate::audio::Player;
use crate::metre::Instant;
use crate::notes::Key;
use crate::popup::Specification;
use crate::project::Settings;
use crate::view::{Axis, OnClick, View};
use crate::{ToArcStr as _, UserInterface};
use arcstr::{ArcStr, literal};

// TODO: add a symbol view instead of using chars
// there is sadly no "single" variant
/// "BLACK LEFT-POINTING DOUBLE TRIANGLE WITH VERTICAL BAR"
const TO_START: ArcStr = literal!(" \u{23EE} ");
/// "BLACK RIGHT-POINTING TRIANGLE"
const PLAY: ArcStr = literal!(" \u{25B6} ");
/// "DOUBLE VERTICAL BAR"
const PAUSE: ArcStr = literal!(" \u{23F8} ");
/// "BLACK CIRCLE FOR RECORD"
const RECORD: ArcStr = literal!(" \u{23FA} ");
/// "CLOCKWISE RIGHTWARDS AND LEFTWARDS OPEN CIRCLE ARROWS"
const LOOP: ArcStr = literal!("loop");
/// "LOWER RIGHT PENCIL"
const EDIT: ArcStr = literal!("edit mode");
/// "MUSICAL KEYBOARD"
const PIANO: ArcStr = literal!("piano roll");
/// "CONTROL KNOBS"
const NODES: ArcStr = literal!("plugins");

fn open_key_selector(instant: Instant, key: Key) -> OnClick {
    OnClick::from(Action::OpenPopup(Specification::KeySelector {
        instant,
        key,
    }))
}

/// The bar att the top of the window.
pub(crate) fn bar<Ui: UserInterface>(
    title: ArcStr,
    settings: &Settings,
    player: Option<Player>,
    edit_mode: bool,
    piano_roll_open: bool,
) -> View {
    let key_button = View::standard_button(
        settings.key.start.to_arc_str(),
        open_key_selector(Instant::START, settings.key.start),
    );
    // TODO: add functionality
    let time_signature_button = View::standard_button(
        settings.time_signature.start.to_arc_str(),
        OnClick::default(),
    );
    // TODO: add functionality
    let tempo_button = View::standard_button(settings.tempo.start.to_arc_str(), OnClick::default());

    let to_start_button =
        View::standard_button(TO_START, OnClick::from(Action::MoveCursor(Instant::START)));
    let playback_button = View::generator(move || {
        let is_playing = player.as_ref().is_some_and(Player::is_playing);

        if is_playing {
            View::standard_button(PAUSE, OnClick::from(Action::Pause))
        } else {
            View::standard_button(PLAY, OnClick::from(Action::Play))
        }
    });
    // TODO: add functionality
    let record_button = View::standard_button(RECORD, OnClick::default());
    // TODO: add functionality
    let loop_button = View::standard_button(LOOP, OnClick::default());

    let edit_mode_button = View::standard_button(EDIT, OnClick::from(Action::ToggleEditMode))
        .with_selection_status(edit_mode);
    let piano_roll_button = View::standard_button(PIANO, OnClick::from(Action::TogglePianoRoll))
        .with_selection_status(piano_roll_open);
    // TODO: add functionality
    let nodes_button = View::standard_button(NODES, OnClick::default());

    // TODO: show current settings?
    let project_settings =
        View::balanced_stack(Axis::X, [key_button, time_signature_button, tempo_button])
            .quotated_minimally();

    let left_playback_buttons = to_start_button.quotated_minimally();

    let right_playback_buttons = View::minimal_stack(Axis::X, [record_button, loop_button]);

    let miscellaneous_buttons =
        View::minimal_stack(Axis::X, [edit_mode_button, piano_roll_button, nodes_button]);

    let left_side = View::x_stack([project_settings, left_playback_buttons]);

    let right_side = View::minimal_stack(Axis::X, [right_playback_buttons, miscellaneous_buttons]);

    View::x_stack([
        left_side.fill_remaining(),
        playback_button.quotated(Ui::PLAYBACK_BUTTON_WIDTH.get()),
        right_side.fill_remaining(),
    ])
    .bordered()
    .titled(title)
    .with_thickness(true)
}
