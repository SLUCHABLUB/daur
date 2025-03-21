use crate::colour::Colour;
use crate::pitch::Pitch;
use crate::view::View;

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
