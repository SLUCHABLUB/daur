use crate::app::Action;
use crate::ui::{Length, Offset, Point, Rectangle};
use crate::view::heterogeneous::ThreeStack;
use crate::view::homogenous::Stack;
use crate::view::{Direction, View};
use crossterm::event::MouseButton;
use educe::Educe;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;
use std::cmp::Ordering;
use std::iter::from_fn;

/// A window into an infinite and scrollable stack
#[derive(Educe)]
#[educe(Debug)]
pub struct Feed<'generator, Child> {
    /// The direction in which the views are laid out
    pub direction: Direction,
    /// How far the feed has been scrolled
    pub offset: Offset,
    /// The functions used for generating the views
    #[educe(Debug(ignore))]
    pub generator: Box<dyn Fn(isize) -> (Child, Length) + 'generator>,
}

// Whilst you could remove the trait bound, since it is not needed for the constructor,
// it helps with type inference. The struct initializer can always be used.
impl<'generator, Child> Feed<'generator, Child> {
    /// Constructs a new feed
    pub fn new<Generator>(direction: Direction, offset: Offset, generator: Generator) -> Self
    where
        Generator: Fn(isize) -> (Child, Length) + 'generator,
    {
        Feed {
            direction,
            offset,
            generator: Box::new(generator),
        }
    }
}

impl<Child: View> View for Feed<'_, Child> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let mut offset = self.offset;

        let first_size;
        let first;

        let mut index = 0;

        match offset.cmp(&Offset::ZERO) {
            Ordering::Less => loop {
                let (child, size) = (self.generator)(index);
                offset += size;
                index = index.saturating_add(1);

                if Offset::ZERO < offset {
                    first = child;
                    first_size = offset.saturate();
                    break;
                }
            },
            Ordering::Equal => {
                let (child, size) = (self.generator)(0);
                first = child;
                first_size = size;
                index = 1;
            }
            Ordering::Greater => loop {
                let new = index.saturating_sub(1);

                let (child, size) = (self.generator)(new);
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
                let (child, size) = (self.generator)(index);

                let new_last_size = last_size - size;

                if new_last_size == Length::ZERO {
                    return None;
                }

                index = index.saturating_add(1);
                last_size = new_last_size;

                Some((child, size.constraint()))
            }),
        );

        let (last, _) = (self.generator)(index);

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
