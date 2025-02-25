use crate::ui::{Length, Offset};
use crate::widget::heterogeneous::ThreeStack;
use crate::widget::homogenous::Stack;
use crate::widget::Direction;
use ratatui::layout::Constraint;
use std::cmp::Ordering;
use std::iter::from_fn;

/// The type returned by [`feed`]
pub type Feed<Child> = ThreeStack<Child, Stack<Child>, Child>;

/// A window into an infinite and scrollable stack
pub fn feed<Child, Generator>(
    direction: Direction,
    mut offset: Offset,
    feed_size: Length,
    mut generator: Generator,
) -> Feed<Child>
where
    Generator: FnMut(isize) -> (Child, Length),
{
    let first_size;
    let first;

    let mut index = 0;

    match offset.cmp(&Offset::ZERO) {
        Ordering::Less => loop {
            let (child, size) = generator(index);
            offset += size;
            index = index.saturating_add(1);

            if Offset::ZERO < offset {
                first = child;
                first_size = offset.saturate();
                break;
            }
        },
        Ordering::Equal => {
            let (child, size) = generator(0);
            first = child;
            first_size = size;
            index = 1;
        }
        Ordering::Greater => loop {
            let new = index.saturating_sub(1);

            let (child, size) = generator(new);
            offset -= size;

            if offset < Offset::ZERO {
                first = child;
                first_size = (offset + size).saturate();
                break;
            }

            index = new;
        },
    }

    let mut last_size = feed_size - first_size;

    let homogeneous = Stack::new(
        direction,
        from_fn(|| {
            let (child, size) = generator(index);

            let new_last_size = last_size - size;

            if new_last_size == Length::ZERO {
                return None;
            }

            index = index.saturating_add(1);
            last_size = new_last_size;

            Some((child, size.constraint()))
        }),
    );

    let (last, _) = generator(index);

    let constraints = [
        first_size.constraint(),
        Constraint::Fill(1),
        last_size.constraint(),
    ];

    ThreeStack::new(direction, (first, homogeneous, last), constraints)
}
