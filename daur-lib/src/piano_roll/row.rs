use crate::pitch::Pitch;
use crate::view::View;
use ratatui::style::Color;

pub fn row(pitch: Pitch) -> View {
    // TODO:
    //  - draw notes
    //  - draw grid
    //  - highlight key based on settings
    let colour = if (pitch - Pitch::A440).semitones() % 2 == 0 {
        Color::Gray
    } else {
        Color::DarkGray
    };

    // TODO: use `Button` for
    //  - adding notes
    //  - selecting notes
    //  - moving the cursor
    View::Solid(colour)
}
