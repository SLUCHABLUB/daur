use crate::key::Key;
use crate::pitch::Pitch;
use crate::ui::{Colour, NonZeroLength};
use crate::view::{Alignment, ToText as _, View};
use arcstr::ArcStr;

// TODO: use `Button` for:
//  - resizing the piano
//  - plinking the key
//  - selecting all notes with the key's pitch
/// Return the view for a key on the piano-roll piano.
pub fn piano_key(pitch: Pitch, key: Key, black_key_depth: NonZeroLength) -> View {
    let top = View::Solid(if pitch.chroma().is_black_key() {
        Colour::BLACK
    } else {
        Colour::WHITE
    });

    let text = if pitch.chroma() == key.tonic {
        ArcStr::from(pitch.name(key.sign))
    } else {
        ArcStr::new()
    }
    .aligned_to(Alignment::BottomRight);

    let bottom = View::Layers(vec![View::Solid(Colour::WHITE), text]);

    View::x_stack([top.quotated(black_key_depth.get()), bottom.fill_remaining()])
}
