use crate::key::Key;
use crate::pitch::Pitch;
use crate::ui::NonZeroLength;
use crate::widget::heterogeneous::{Layers, TwoStack};
use crate::widget::{Solid, Text, ToWidget};
use arcstr::ArcStr;
use ratatui::layout::Constraint;

#[derive(Copy, Clone, Debug)]
pub struct PianoKey {
    pub key: Key,
    pub pitch: Pitch,
    pub black_key_depth: NonZeroLength,
}

impl ToWidget for PianoKey {
    // TODO: use `Button` for:
    //  - resizing the piano
    //  - plinking the key
    //  - selecting all notes with the keys pitch
    type Widget<'widget> = TwoStack<Solid, Layers<(Solid, Text)>>;

    fn to_widget(&self) -> Self::Widget<'_> {
        let top = if self.pitch.chroma().is_black_key() {
            Solid::BLACK
        } else {
            Solid::WHITE
        };

        let text = Text::bottom_right(if self.pitch.chroma() == self.key.tonic {
            ArcStr::from(self.pitch.name(self.key.sign))
        } else {
            ArcStr::new()
        });

        let bottom = Layers::new((Solid::WHITE, text));

        let constraints = [self.black_key_depth.get().constraint(), Constraint::Fill(1)];

        TwoStack::horizontal((top, bottom), constraints)
    }
}
