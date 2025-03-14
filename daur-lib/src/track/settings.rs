use crate::app::Action;
use crate::track::Track;
use crate::view::{Bordered, Button, Composition, OnClick, Text};
use arcstr::{literal, ArcStr};
use std::sync::Arc;

#[derive(Debug)]
pub struct Settings {
    pub track: Arc<Track>,
    pub selected: bool,
    pub index: usize,
}

impl Composition for Settings {
    type Body<'view> = Button<'static, Bordered<Text>>;

    fn body(&self) -> Self::Body<'_> {
        let title = ArcStr::clone(&self.track.name);
        let on_click = OnClick::from(Action::SelectTrack(self.index));

        Button {
            on_click,
            content: Bordered::titled(title, Text::centred(literal!("TODO")))
                .thickness(self.selected),
        }
    }
}
