use crate::audio::Player;
use crate::musical_time::Instant;
use crate::pitch::Pitch;
use crate::project;
use crate::ui::{Colour, Grid};
use crate::view::piano_roll::Settings;
use crate::view::{CursorWindow, View};

/// Return the view for a (non-piano) row of the piano roll.
pub(crate) fn row(
    pitch: Pitch,
    piano_roll_settings: &Settings,
    project_settings: project::Settings,
    grid: Grid,
    player: Option<Player>,
    cursor: Instant,
) -> View {
    // TODO:
    //  - draw notes
    //  - draw grid
    //  - highlight key based on settings
    let colour = if (pitch - Pitch::A440).semitones() % 2 == 0 {
        Colour::gray_scale(0xAA)
    } else {
        Colour::gray_scale(0x55)
    };

    // TODO: use `Button` for
    //  - adding notes
    //  - selecting notes
    //  - moving the cursor
    // TODO: move the cursor window up (so there is only one cursor window)
    View::Layers(vec![
        View::Solid(colour),
        CursorWindow::view(
            player,
            cursor,
            project_settings,
            grid,
            piano_roll_settings.x_offset,
        ),
    ])
}
