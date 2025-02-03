use crate::app::action::Action;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use saturating_cast::SaturatingCast;
use std::iter::zip;

pub struct HomogenousStack<T> {
    direction: Direction,
    children: Vec<(T, Constraint)>,
}

impl<T> HomogenousStack<T> {
    pub fn vertical<Children: IntoIterator<Item = (T, Constraint)>>(children: Children) -> Self {
        HomogenousStack {
            direction: Direction::Vertical,
            children: children.into_iter().collect(),
        }
    }

    pub fn equidistant_vertical<Children>(children: Children) -> Self
    where
        Children: IntoIterator<Item = T>,
        Children::IntoIter: ExactSizeIterator,
    {
        let children = children.into_iter();
        let length = children.len().saturating_cast();
        // Using a ratio of 1 / length is faster than using a fill of 1
        HomogenousStack::vertical(children.map(|child| (child, Constraint::Ratio(1, length))))
    }

    fn unzip_children(&self) -> (Vec<&T>, Vec<Constraint>) {
        self.children
            .iter()
            .map(|(child, constraint)| (child, constraint))
            .unzip()
    }
}

impl<T: Widget> Widget for HomogenousStack<T> {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let (children, constraints) = self.unzip_children();
        let areas = Layout::new(self.direction, constraints).split(area);

        for (child, area) in zip(children, areas.iter()) {
            child.render(*area, buf, mouse_position);
        }
    }

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
        let (children, constraints) = self.unzip_children();
        let areas = Layout::new(self.direction, constraints).split(area);

        for (child, area) in zip(children, areas.iter()) {
            if area.contains(position) {
                child.click(*area, button, position, action_queue);
            }
        }
    }
}
