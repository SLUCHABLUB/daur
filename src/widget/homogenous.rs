use crate::app::Action;
use crate::length::offset::Offset;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::length::size::Size;
use crate::length::Length;
use crate::widget::has_size::HasSize;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Flex, Spacing};
use saturating_cast::SaturatingCast as _;
use std::iter::zip;

pub struct Stack<T> {
    direction: Direction,
    children: Vec<(T, Constraint)>,
    spacing: Spacing,
    flex: Flex,
}

impl<T> Stack<T> {
    pub fn horizontal<Children: IntoIterator<Item = (T, Constraint)>>(children: Children) -> Self {
        Stack {
            direction: Direction::Horizontal,
            children: children.into_iter().collect(),
            spacing: Spacing::default(),
            flex: Flex::SpaceBetween,
        }
    }

    pub fn vertical<Children: IntoIterator<Item = (T, Constraint)>>(children: Children) -> Self {
        Stack {
            direction: Direction::Vertical,
            children: children.into_iter().collect(),
            spacing: Spacing::default(),
            flex: Flex::SpaceBetween,
        }
    }

    pub fn horizontal_sized<Children: IntoIterator<Item = T>>(children: Children) -> Self
    where
        T: HasSize,
    {
        Stack::horizontal(children.into_iter().map(|child| {
            let constraint = child.size().width.constraint();
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
        Stack::vertical(children.map(|child| (child, Constraint::Ratio(1, length))))
    }

    pub fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    pub fn spacing<S: Into<Spacing>>(mut self, spacing: S) -> Self {
        self.spacing = spacing.into();
        self
    }

    fn areas(
        &self,
        area: Rectangle,
        constraints: Vec<Constraint>,
    ) -> impl Iterator<Item = Rectangle> {
        area.split(constraints, self.direction, self.flex, &self.spacing)
    }

    fn unzip_children(&self) -> (Vec<&T>, Vec<Constraint>) {
        self.children
            .iter()
            .map(|(child, constraint)| (child, constraint))
            .unzip()
    }
}

impl<T: Widget> Widget for Stack<T> {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        let (children, constraints) = self.unzip_children();
        let areas = self.areas(area, constraints);

        for (child, area) in zip(children, areas) {
            child.render(area, buf, mouse_position);
        }
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        let (children, constraints) = self.unzip_children();
        let areas = self.areas(area, constraints);

        for (child, area) in zip(children, areas) {
            if area.contains(position) {
                child.click(area, button, position, actions);
            }
        }
    }
}

impl<T: HasSize> HasSize for Stack<T> {
    fn size(&self) -> Size {
        let mut parallel = Length::ZERO;
        let mut orthogonal = Length::ZERO;

        for (child, _) in &self.children {
            let child = child.size();
            parallel += child.parallel_to(self.direction);
            orthogonal = Length::max(orthogonal, child.orthogonal_to(self.direction));
        }

        let space_count = self.children.len().saturating_sub(1);
        parallel += Offset::from(&self.spacing) * space_count;

        Size::from_parallel_orthogonal(parallel, orthogonal, self.direction)
    }
}
