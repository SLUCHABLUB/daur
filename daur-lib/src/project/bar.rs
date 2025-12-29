use crate::Project;
use crate::ToArcStr as _;
use crate::UserInterface;
use crate::app::Action;
use crate::audio::Player;
use crate::metre::Instant;
use crate::popup::Specification;
use crate::view::Axis;
use crate::view::OnClick;
use crate::view::ToText as _;
use crate::view::View;
use arcstr::ArcStr;
use arcstr::literal;

// TODO: add a symbol view instead of using chars
// there is sadly no "single" variant
/// "BLACK LEFT-POINTING DOUBLE TRIANGLE WITH VERTICAL BAR"
const BACK: ArcStr = literal!(" \u{23EE} ");
/// "DOUBLE VERTICAL BAR"
const PAUSE: ArcStr = literal!(" \u{23F8} ");
/// "BLACK RIGHT-POINTING TRIANGLE"
const PLAY: ArcStr = literal!(" \u{25B6} ");
/// "BLACK CIRCLE FOR RECORD"
const RECORD: ArcStr = literal!(" \u{23FA} ");

const EDIT: ArcStr = literal!("edit mode");
const EXPORT: ArcStr = literal!("export");
const LOOP: ArcStr = literal!("loop");
const PIANO: ArcStr = literal!("piano roll");
const PLUGINS: ArcStr = literal!("plugins");
const SETTINGS: ArcStr = literal!("settings");

/// The bar att the top of the window.
pub(crate) fn bar<Ui: UserInterface>(
    project: &Project,
    cursor: Instant,
    player: Option<Player>,
    edit_mode: bool,
    piano_roll_open: bool,
) -> View {
    // --- BUTTONS ---

    // TODO: add functionality
    let plugins_button = View::standard_button(PLUGINS, OnClick::default());
    let piano_roll_button = View::toggle(
        PIANO,
        OnClick::from(Action::TogglePianoRoll),
        piano_roll_open,
    );
    let edit_mode_button = View::toggle(EDIT, OnClick::from(Action::ToggleEditMode), edit_mode);

    let key_button = View::standard_button(
        project.key.start.to_arc_str(),
        OnClick::from(Action::OpenPopup(Specification::KeySelector {
            key: project.key.get(cursor),
        })),
    );
    // TODO: add functionality
    let time_signature_button = View::standard_button(
        project.time_signature.get(cursor).to_arc_str(),
        OnClick::default(),
    );
    // TODO: add functionality
    let tempo_button =
        View::standard_button(project.tempo.get(cursor).to_arc_str(), OnClick::default());

    let back_button =
        View::standard_button(BACK, OnClick::from(Action::MoveCursor(Instant::START)));
    let playback_button = View::reactive(move |_| {
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

    // TODO: add functionality
    let loudness_metre = literal!("0 LUFS").centred().bordered();

    // TODO: display if the project haas been exported
    let export_button = View::standard_button(EXPORT, OnClick::from(Action::ExportProject));
    // TODO: add functionality
    let settings_button = View::standard_button(SETTINGS, OnClick::default());

    // --- BUTTON CLUSTERS ---

    let leftmost_buttons = View::minimal_stack(
        Axis::X,
        [plugins_button, piano_roll_button, edit_mode_button],
    );

    let project_settings =
        View::balanced_stack(Axis::X, [key_button, time_signature_button, tempo_button]);

    let left_playback_buttons = back_button;

    let right_playback_buttons = View::minimal_stack(Axis::X, [record_button, loop_button]);

    let rightmost_buttons = View::minimal_stack(Axis::X, [export_button, settings_button]);

    let left_side = View::minimal_stack(
        Axis::X,
        [leftmost_buttons, project_settings, left_playback_buttons],
    );

    let right_side = View::minimal_stack(
        Axis::X,
        [right_playback_buttons, loudness_metre, rightmost_buttons],
    );

    View::x_stack([
        left_side.fill_remaining(),
        playback_button.quotated(Ui::PLAYBACK_BUTTON_WIDTH),
        right_side.fill_remaining(),
    ])
    .bordered_with_title_and_thickness(project.name.clone(), true)
}
