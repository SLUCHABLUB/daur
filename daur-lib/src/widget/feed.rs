use crate::app::Action;
use crate::ui::{Length, Offset, Point, Rectangle};
use crate::widget::heterogeneous::ThreeStack;
use crate::widget::homogenous::Stack;
use crate::widget::{Direction, Widget};
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;
use std::cmp::Ordering;
use std::iter::from_fn;

/// A window into an infinite and scrollable stack
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Feed<Children> {
    /// The direction in which the widgets are laid out
    pub direction: Direction,
    /// How far the feed has been scrolled
    pub offset: Offset,
    /// The functions used for generating the widgets
    pub children: Children,
}

// Whilst you could remove the trait bound, since it is not needed for the constructor,
// it helps with type inference. The struct initializer can always be used.
impl<Children: Fn(isize) -> Child, Child> Feed<Children> {
    /// Constructs a new feed
    pub fn new(direction: Direction, offset: Offset, children: Children) -> Feed<Children> {
        Feed {
            direction,
            offset,
            children,
        }
    }
}

impl<Child: Widget, Children> Widget for Feed<Children>
where
    Children: Fn(isize) -> (Child, Length),
{
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let mut offset = self.offset;

        let first_size;
        let first;

        let mut index = 0;

        match offset.cmp(&Offset::ZERO) {
            Ordering::Less => loop {
                let (child, size) = (self.children)(index);
                offset += size;
                index = index.saturating_add(1);

                if Offset::ZERO < offset {
                    first = child;
                    first_size = offset.saturate();
                    break;
                }
            },
            Ordering::Equal => {
                let (child, size) = (self.children)(0);
                first = child;
                first_size = size;
                index = 1;
            }
            Ordering::Greater => loop {
                let new = index.saturating_sub(1);

                let (child, size) = (self.children)(new);
                offset -= size;

                if offset < Offset::ZERO {
                    first = child;
                    first_size = (offset + size).saturate();
                    break;
                }

                index = new;
            },
        }

        let mut last_size = area.size.parallel_to(self.direction) - first_size;

        let homogeneous = Stack::new(
            self.direction,
            from_fn(|| {
                let (child, size) = (self.children)(index);

                let new_last_size = last_size - size;

                if new_last_size == Length::ZERO {
                    return None;
                }

                index = index.saturating_add(1);
                last_size = new_last_size;

                Some((child, size.constraint()))
            }),
        );

        let (last, _) = (self.children)(index);

        let constraints = [
            first_size.constraint(),
            Constraint::Fill(1),
            last_size.constraint(),
        ];

        ThreeStack::new(self.direction, (first, homogeneous, last), constraints).render(
            area,
            buffer,
            mouse_position,
        );
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {}
}
