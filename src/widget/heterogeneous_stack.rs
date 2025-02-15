use crate::app::action::Action;
use crate::widget::block::Block;
use crate::widget::sized::{join, split, Sized};
use crate::widget::to_widget::ToWidget;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Position, Rect, Size, Spacing};
use ratatui::widgets;
use std::rc::Rc;

pub type TwoStack<A, B> = HeterogeneousStack<2, (A, B)>;
pub type ThreeStack<A, B, C> = HeterogeneousStack<3, (A, B, C)>;
pub type FourStack<A, B, C, D> = HeterogeneousStack<4, (A, B, C, D)>;

pub struct HeterogeneousStack<const N: usize, Children> {
    direction: Direction,
    children: Children,
    constraints: [Constraint; N],
    block: Option<Block>,
    flex: Flex,
    spacing: Spacing,
}

impl<const N: usize, Children> HeterogeneousStack<N, Children> {
    pub fn block(mut self, block: Block) -> Self {
        self.block = Some(block);
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

    fn inner_area(&self, area: Rect) -> Rect {
        if self.block.is_some() {
            widgets::Block::bordered().inner(area)
        } else {
            area
        }
    }

    fn areas(&self, area: Rect) -> Rc<[Rect]> {
        Layout::new(self.direction, self.constraints)
            .flex(self.flex)
            .spacing(self.spacing.clone())
            .split(area)
    }
}

macro_rules! impl_hetero {
    ($len:literal; $($generic:ident),*; $($index:tt),*) => {
        #[allow(dead_code)]
        impl<$($generic),*> HeterogeneousStack<$len, ($($generic),*)> {
            fn new(
                direction: Direction,
                children: ($($generic),*),
                constraints: [Constraint; $len],
            ) -> Self {
                HeterogeneousStack {
                    direction,
                    children,
                    constraints,
                    block: None,
                    flex: Flex::default(),
                    spacing: Spacing::default(),
                }
            }

            pub fn horizontal(
                children: ($($generic),*),
                constraints: [Constraint; $len],
            ) -> Self {
                Self::new(Direction::Horizontal, children, constraints)
            }

            pub fn vertical(
                children: ($($generic),*),
                constraints: [Constraint; $len],
            ) -> Self {
                Self::new(Direction::Vertical, children, constraints)
            }
        }

        impl<$($generic: Widget),*> Widget for HeterogeneousStack<$len, ($($generic),*)> {
            fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
                if let Some(block) = self.block.as_ref() {
                    block.to_widget().render(area, buf, mouse_position);
                }
                let areas = self.areas(self.inner_area(area));

                $(
                    self.children.$index.render(areas[$index], buf, mouse_position);
                )*
            }

            fn click(
                &self,
                area: Rect,
                button: MouseButton,
                position: Position,
                action_queue: &mut Vec<Action>,
            ) {
                let areas = self.areas(self.inner_area(area));
                $(
                    if areas[$index].contains(position) {
                        self.children.$index.click(areas[$index], button, position, action_queue);
                    }
                )*
            }
        }

        impl<$($generic: Sized),*> Sized for HeterogeneousStack<$len, ($($generic),*)> {
            fn size(&self) -> Size {
                let mut dominant = 0;
                let mut non_dominant = 0;

                if self.block.is_some() {
                    dominant += 2;
                    non_dominant += 2;
                }

                let space_count = $len - 1;
                match self.spacing {
                    Spacing::Space(space) => dominant += space_count * space,
                    Spacing::Overlap(space) => dominant = dominant.saturating_sub(space_count * space),
                }

                $(
                    let [dom, non] = split(self.children.$index.size(), self.direction);
                    dominant += dom;
                    non_dominant = u16::max(non_dominant, non);
                )*

                join(dominant, non_dominant, self.direction)
            }
        }
    }
}

impl_hetero!(2; A, B; 0, 1);
impl_hetero!(3; A, B, C; 0, 1, 2);
impl_hetero!(4; A, B, C, D; 0, 1, 2, 3);
