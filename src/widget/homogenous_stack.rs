use crate::app::action::Action;
use crate::length::offset::Offset;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::length::size::Size;
use crate::length::Length;
use crate::widget::sized::{join, split, Sized};
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Flex, Spacing};
use saturating_cast::SaturatingCast as _;
use std::iter::zip;

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
        HomogenousStack::vertical(children.map(|child| (child, Constraint::Ratio(1, length))))
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

impl<T: Widget> Widget for HomogenousStack<T> {
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

impl<T: Sized> Sized for HomogenousStack<T> {
    fn size(&self) -> Size {
        let mut dominant = Length::ZERO;
        let mut non_dominant = Length::ZERO;

        for (child, _) in &self.children {
            let [dom, non] = split(child.size(), self.direction);
            dominant += dom;
            non_dominant = Length::max(non_dominant, non);
        }

        let space_count = self.children.len().saturating_sub(1);
        dominant += Offset::from(&self.spacing) * space_count;

        join(dominant, non_dominant, self.direction)
    }
}
