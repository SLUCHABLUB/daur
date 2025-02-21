use crate::app::Action;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::track::Track;
use crate::widget::bordered::Bordered;
use crate::widget::text::Text;
use crate::widget::Widget;
use arcstr::{literal, ArcStr};
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use std::sync::Arc;

pub struct Settings {
    pub track: Arc<Track>,
    pub selected: bool,
    pub index: usize,
}

impl Settings {
    fn visual(&self) -> impl Widget {
        Bordered::new(
            ArcStr::clone(&self.track.name),
            Text::centered(literal!("TODO")),
            self.selected,
        )
    }
}

impl Widget for Settings {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        self.visual().render(area, buf, mouse_position);
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
