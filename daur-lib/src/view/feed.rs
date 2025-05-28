use crate::UserInterface;
use crate::ui::{Direction, Length, Offset};
use crate::view::{Quotated, View};
use std::cmp::Ordering;

/// A window into an infinite and scrollable stack
pub(crate) fn feed<Ui: UserInterface, Generator>(
    direction: Direction,
    offset: Offset,
    generator: Generator,
) -> View
where
    Generator: Fn(isize) -> Quotated + Send + Sync + 'static,
{
    View::reactive(move |render_area| {
        let mut offset = offset;

        let full_size = render_area.area.size.parallel_to(direction.axis());

        let first;

        let mut index = 0;

        match offset.cmp(&Offset::ZERO) {
            Ordering::Less => loop {
                let quotated = generator(index);
                offset += quotated
                    .size_parallel_to::<Ui>(direction.axis(), render_area)
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

                let quotated = generator(new);
                let quotum_size = quotated.size_parallel_to::<Ui>(direction.axis(), render_area);
                offset -= quotum_size.unwrap_or(full_size);

                if offset < Offset::ZERO {
                    first = if let Some(size) = quotum_size {
                        quotated.view.quotated((offset + size).rectify())
                    } else {
                        quotated.view.fill_remaining()
                    };
                    break;
                }

                index = new;
            },
        }

        let used_size = first
            .size_parallel_to::<Ui>(direction.axis(), render_area)
            .unwrap_or(full_size);

        let mut remaining = full_size - used_size;

        let mut elements = vec![first];

        while remaining != Length::ZERO {
            let quotated = generator(index);

            let new_remaining = remaining
                - quotated
                    .size_parallel_to::<Ui>(direction.axis(), render_area)
                    .unwrap_or(full_size);

            index = index.saturating_add(1);
            remaining = new_remaining;

            elements.push(quotated);
        }

        if direction.is_negative() {
            elements.reverse();
        }

        View::Stack {
            axis: direction.axis(),
            elements,
        }
    })
}
