use crate::app::action::Action;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Direction, Flex, Layout, Position, Rect, Spacing};
use ratatui::prelude::Constraint;
use ratatui::widgets::Block;
use std::rc::Rc;

pub struct ThreeStack<'a, A, B, C> {
    direction: Direction,
    children: (A, B, C),
    constraints: [Constraint; 3],
    block: Block<'a>,
    spacing: Spacing,
    flex: Flex,
}

impl<'a, A, B, C> ThreeStack<'a, A, B, C> {
    pub fn horizontal(children: (A, B, C), constraints: [Constraint; 3]) -> Self {
        ThreeStack {
            direction: Direction::Horizontal,
            children,
            constraints,
            block: Block::default(),
            spacing: Spacing::default(),
            flex: Flex::default(),
        }
    }

    pub fn vertical(children: (A, B, C), constraints: [Constraint; 3]) -> Self {
        ThreeStack {
            direction: Direction::Vertical,
            children,
            constraints,
            block: Block::default(),
            spacing: Spacing::default(),
            flex: Flex::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = block;
        self
    }

    pub fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Spacing>) -> Self {
        self.spacing = spacing.into();
        self
    }

    fn areas(&self, area: Rect) -> Rc<[Rect]> {
        // `Spacing` should implement copy but doesn't for some reason
        Layout::new(self.direction, self.constraints)
            .flex(self.flex)
            .spacing(self.spacing.clone())
            .split(self.block.inner(area))
    }
}

impl<A: Widget, B: Widget, C: Widget> Widget for ThreeStack<'_, A, B, C> {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        self.block.render(area, buf, mouse_position);
        let areas = self.areas(area);
        self.children.0.render(areas[0], buf, mouse_position);
        self.children.1.render(areas[1], buf, mouse_position);
        self.children.2.render(areas[2], buf, mouse_position);
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
        if areas[2].contains(position) {
            self.children.2.click(area, button, position, action_queue);
        }
    }
}
