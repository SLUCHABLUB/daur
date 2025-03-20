use crate::key::Key;
use crate::pitch::Pitch;
use crate::ui::NonZeroLength;
use crate::view::{Direction, View};
use arcstr::ArcStr;
use ratatui::style::Color;

// TODO: use `Button` for:
//  - resizing the piano
//  - plinking the key
//  - selecting all notes with the keys pitch
pub fn piano_key(pitch: Pitch, key: Key, black_key_depth: NonZeroLength) -> View {
    let top = View::Solid(if pitch.chroma().is_black_key() {
        Color::Black
    } else {
        Color::White
    });

    let text = View::bottom_right(if pitch.chroma() == key.tonic {
        ArcStr::from(pitch.name(key.sign))
    } else {
        ArcStr::new()
    });

    let bottom = View::Layers(vec![View::Solid(Color::White), text]);

    View::Stack {
        direction: Direction::Right,
        elements: vec![top.quotated(black_key_depth.get()), bottom.fill_remaining()],
    }
}
