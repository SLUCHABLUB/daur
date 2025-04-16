use crate::ui::{Length, Offset};
use crate::view::{Direction, Quotated, View};
use std::cmp::Ordering;

/// A window into an infinite and scrollable stack
pub fn feed<Generator>(direction: Direction, offset: Offset, generator: Generator) -> View
where
    Generator: Fn(isize) -> Quotated + Send + Sync + 'static,
{
    View::size_informed(move |size| {
        let mut offset = offset;

        let full_size = size.parallel_to(direction);

        let first;

        let mut index = 0;

        match offset.cmp(&Offset::ZERO) {
            Ordering::Less => loop {
                let quotated = generator(index);
                offset += quotated
                    .quotum
                    .size_parallel_to(direction)
                    .unwrap_or(full_size);
                index = index.saturating_add(1);

                if Offset::ZERO < offset {
                    first = quotated.view.quotated(offset.rectify());
                    break;
                }
            },
            Ordering::Equal => {
                first = generator(0);
                index = 1;
            }
            Ordering::Greater => loop {
                let new = index.saturating_sub(1);

                let Quotated { quotum, view } = generator(new);
                let quotum_size = quotum.size_parallel_to(direction);
                offset -= quotum_size.unwrap_or(full_size);

                if offset < Offset::ZERO {
                    first = if let Some(size) = quotum_size {
                        view.quotated((offset + size).rectify())
                    } else {
                        view.fill_remaining()
                    };
                    break;
                }

                index = new;
            },
        }

        let used_size = first
            .quotum
            .size_parallel_to(direction)
            .unwrap_or(full_size);

        let mut remaining = full_size - used_size;

        let mut elements = vec![first];

        while remaining != Length::ZERO {
            let quotated = generator(index);

            let new_remaining = remaining
                - quotated
                    .quotum
                    .size_parallel_to(direction)
                    .unwrap_or(full_size);

            index = index.saturating_add(1);
            remaining = new_remaining;

            elements.push(quotated);
        }

        View::Stack {
            direction,
            elements,
        }
    })
}
