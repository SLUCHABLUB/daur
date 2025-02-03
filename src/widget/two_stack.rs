use crate::app::action::Action;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Direction, Layout, Position, Rect};
use ratatui::prelude::Constraint;

pub struct TwoStack<A, B> {
    direction: Direction,
    children: (A, B),
    constraints: [Constraint; 2],
}

impl<A, B> TwoStack<A, B> {
    pub fn horizontal(children: (A, B), constraints: [Constraint; 2]) -> Self {
        TwoStack {
            direction: Direction::Horizontal,
            children,
            constraints,
        }
    }

    pub fn vertical(children: (A, B), constraints: [Constraint; 2]) -> Self {
        TwoStack {
            direction: Direction::Vertical,
            children,
            constraints,
        }
    }
}

impl<A: Widget, B: Widget> Widget for TwoStack<A, B> {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let areas = Layout::new(self.direction, self.constraints).split(area);
        self.children.0.render(areas[0], buf, mouse_position);
        self.children.1.render(areas[1], buf, mouse_position);
    }

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
        let areas = Layout::new(self.direction, self.constraints).split(area);
        if areas[0].contains(position) {
            self.children.0.click(area, button, position, action_queue);
        }
        if areas[1].contains(position) {
            self.children.1.click(area, button, position, action_queue);
        }
    }
}
