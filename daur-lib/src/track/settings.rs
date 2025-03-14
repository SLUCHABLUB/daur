use crate::app::Action;
use crate::track::Track;
use crate::widget::{Bordered, Button, OnClick, Text, ToWidget};
use arcstr::{literal, ArcStr};
use std::sync::Arc;

#[derive(Debug)]
pub struct Settings {
    pub track: Arc<Track>,
    pub selected: bool,
    pub index: usize,
}

impl ToWidget for Settings {
    type Widget<'widget> = Button<'static, Bordered<Text>>;

    fn to_widget(&self) -> Self::Widget<'_> {
        let title = ArcStr::clone(&self.track.name);
        let on_click = OnClick::from(Action::SelectTrack(self.index));

        Button {
            on_click,
            content: Bordered::titled(title, Text::centred(literal!("TODO")))
                .thickness(self.selected),
        }
    }
}
