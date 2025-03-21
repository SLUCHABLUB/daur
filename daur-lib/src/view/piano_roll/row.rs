use crate::pitch::Pitch;
use crate::view::View;
use crate::Colour;

/// Return the view for a (non-piano) row of the piano roll.
pub fn row(pitch: Pitch) -> View {
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
    View::Solid(colour)
}
