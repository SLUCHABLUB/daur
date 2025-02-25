//! Homogenous stacks of widgets

use crate::app::Action;
use crate::ui::{Length, Offset, Point, Rectangle, Size};
use crate::widget::has_size::HasSize;
use crate::widget::{Direction, Widget};
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Spacing};
use saturating_cast::SaturatingCast as _;
use std::iter::zip;

/// A homogenous stack of widgets
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Stack<Child> {
    direction: Direction,
    children: Vec<(Child, Constraint)>,
    spacing: Spacing,
    flex: Flex,
}

impl<Child> Stack<Child> {
    /// Constructs a new stack
    #[must_use]
    pub fn new<Children: IntoIterator<Item = (Child, Constraint)>>(
        direction: Direction,
        children: Children,
    ) -> Self {
        Stack {
            direction,
            children: children.into_iter().collect(),
            spacing: Spacing::default(),
            flex: Flex::SpaceBetween,
        }
    }

    /// Constructs a horizontal stack
    #[must_use]
    pub fn horizontal<Children: IntoIterator<Item = (Child, Constraint)>>(
        children: Children,
    ) -> Self {
        Stack::new(Direction::Right, children)
    }

    /// Constructs a vertical stack
    #[must_use]
    pub fn vertical<Children: IntoIterator<Item = (Child, Constraint)>>(
        children: Children,
    ) -> Self {
        Stack::new(Direction::Down, children)
    }

    /// Constructs a horizontal stack where all widgets have a _"canonical"_ size
    #[must_use]
    pub fn horizontal_sized<Children: IntoIterator<Item = Child>>(children: Children) -> Self
    where
        Child: HasSize,
    {
        Stack::horizontal(children.into_iter().map(|child| {
            let constraint = child.size().width.constraint();
            (child, constraint)
        }))
    }

    /// Constructs a vertical stack where all widgets have a _"canonical"_ size
    #[must_use]
    pub fn vertical_sized<Children: IntoIterator<Item = Child>>(children: Children) -> Self
    where
        Child: HasSize,
    {
        Stack::vertical(children.into_iter().map(|child| {
            let constraint = child.size().height.constraint();
            (child, constraint)
        }))
    }

    /// Constructs a horizontal stack of widgets that all have the same size
    #[must_use]
    pub fn equidistant_horizontal<Children>(children: Children) -> Self
    where
        Children: IntoIterator<Item = Child>,
        Children::IntoIter: ExactSizeIterator,
    {
        let children = children.into_iter();
        let length = children.len().saturating_cast();
        // Using a ratio of 1 / ui is faster than using a fill of 1
        Stack::horizontal(children.map(|child| (child, Constraint::Ratio(1, length))))
    }

    /// Constructs a vertical stack of widgets that all have the same size
    #[must_use]
    pub fn equidistant_vertical<Children>(children: Children) -> Self
    where
        Children: IntoIterator<Item = Child>,
        Children::IntoIter: ExactSizeIterator,
    {
        let children = children.into_iter();
        let length = children.len().saturating_cast();
        // Using a ratio of 1 / ui is faster than using a fill of 1
        Stack::vertical(children.map(|child| (child, Constraint::Ratio(1, length))))
    }

    /// Sets the flex between widgets
    #[must_use]
    pub fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    /// Sets the spacing between widgets
    #[must_use]
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

    fn unzip_children(&self) -> (Vec<&Child>, Vec<Constraint>) {
        self.children
            .iter()
            .map(|(child, constraint)| (child, constraint))
            .unzip()
    }
}

impl<Child: Widget> Widget for Stack<Child> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let (children, constraints) = self.unzip_children();
        let areas = self.areas(area, constraints);

        for (child, area) in zip(children, areas) {
            child.render(area, buffer, mouse_position);
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

impl<Child: HasSize> HasSize for Stack<Child> {
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
