use crate::app::Action;
use crate::track::Track;
use crate::ui::{Point, Rectangle};
use crate::widget::{Bordered, Text, Widget};
use arcstr::{literal, ArcStr};
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use std::sync::Arc;

#[derive(Debug)]
pub struct Settings {
    pub track: Arc<Track>,
    pub selected: bool,
    pub index: usize,
}

impl Settings {
    fn visual(&self) -> impl Widget {
        let title = ArcStr::clone(&self.track.name);

        Bordered::new(title, Text::centred(literal!("TODO")), self.selected)
    }
}

impl Widget for Settings {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        self.visual().render(area, buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        actions.push(Action::SelectTrack(self.index));
        self.visual().click(area, button, position, actions);
    }
}
