use crate::app::action::Action;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Direction, Flex, Layout, Position, Rect};
use ratatui::prelude::Constraint;
use std::rc::Rc;

pub struct TwoStack<A, B> {
    direction: Direction,
    children: (A, B),
    constraints: [Constraint; 2],
    flex: Flex,
}

impl<A, B> TwoStack<A, B> {
    pub fn horizontal(children: (A, B), constraints: [Constraint; 2]) -> Self {
        TwoStack {
            direction: Direction::Horizontal,
            children,
            constraints,
            flex: Flex::default(),
        }
    }

    pub fn vertical(children: (A, B), constraints: [Constraint; 2]) -> Self {
        TwoStack {
            direction: Direction::Vertical,
            children,
            constraints,
            flex: Flex::default(),
        }
    }

    pub fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    fn areas(&self, area: Rect) -> Rc<[Rect]> {
        Layout::new(self.direction, self.constraints)
            .flex(self.flex)
            .split(area)
    }
}

impl<A: Widget, B: Widget> Widget for TwoStack<A, B> {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let areas = self.areas(area);
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
        let areas = self.areas(area);
        if areas[0].contains(position) {
            self.children.0.click(area, button, position, action_queue);
        }
        if areas[1].contains(position) {
            self.children.1.click(area, button, position, action_queue);
        }
    }
}
