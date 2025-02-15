use crate::app::action::Action;
use crate::widget::sized::{join, split, Sized};
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Position, Rect, Size, Spacing};
use saturating_cast::SaturatingCast;
use std::iter::zip;
use std::rc::Rc;

// TODO: should we default to `Flex::default()`?
pub struct HomogenousStack<T> {
    direction: Direction,
    children: Vec<(T, Constraint)>,
    spacing: Spacing,
    flex: Flex,
}

impl<T> HomogenousStack<T> {
    pub fn horizontal<Children: IntoIterator<Item = (T, Constraint)>>(children: Children) -> Self {
        HomogenousStack {
            direction: Direction::Horizontal,
            children: children.into_iter().collect(),
            spacing: Spacing::default(),
            flex: Flex::default(),
        }
    }

    pub fn vertical<Children: IntoIterator<Item = (T, Constraint)>>(children: Children) -> Self {
        HomogenousStack {
            direction: Direction::Vertical,
            children: children.into_iter().collect(),
            spacing: Spacing::default(),
            flex: Flex::default(),
        }
    }

    pub fn horizontal_sized<Children: IntoIterator<Item = T>>(children: Children) -> Self
    where
        T: Sized,
    {
        HomogenousStack::horizontal(children.into_iter().map(|child| {
            let constraint = Constraint::Length(child.size().width);
            (child, constraint)
        }))
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

    pub fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Spacing>) -> Self {
        self.spacing = spacing.into();
        self
    }

    fn areas(&self, area: Rect, constraints: Vec<Constraint>) -> Rc<[Rect]> {
        Layout::new(self.direction, constraints)
            .spacing(self.spacing.clone())
            .flex(self.flex)
            .split(area)
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
        let areas = self.areas(area, constraints);

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
        let areas = self.areas(area, constraints);

        for (child, area) in zip(children, areas.iter()) {
            if area.contains(position) {
                child.click(*area, button, position, action_queue);
            }
        }
    }
}

impl<T: Sized> Sized for HomogenousStack<T> {
    fn size(&self) -> Size {
        let mut dominant = 0;
        let mut non_dominant = 0;

        for (child, _) in &self.children {
            let [dom, non] = split(child.size(), self.direction);
            dominant += dom;
            non_dominant = u16::max(non_dominant, non);
        }

        let space_count: u16 = self.children.len().saturating_sub(1).saturating_cast();
        match self.spacing {
            Spacing::Space(space) => dominant += space * space_count,
            Spacing::Overlap(space) => dominant = dominant.saturating_sub(space * space_count),
        }

        join(dominant, non_dominant, self.direction)
    }
}
