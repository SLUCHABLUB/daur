use crate::ui::{Mapping, Offset};
use crate::view::{feed, Direction, View};
use std::num::NonZeroU32;

// TODO: use `Button` for moving and scaling the overview
/// A ruler of musical time
pub fn ruler(mapping: Mapping, offset: Offset) -> View {
    feed(Direction::Right, offset, move |index| {
        if let Ok(bar_index) = usize::try_from(index) {
            let bar = mapping.time_signature.bar_n(bar_index);
            let bar_width = mapping.bar_width(bar);
            let cell_width = mapping.grid.cell_width;

            let cells = (bar_width / cell_width).ceil();
            let Some(cells) = NonZeroU32::new(cells) else {
                return View::EMPTY.quotated(bar_width);
            };

            View::Rule { index, cells }.quotated(bar_width)
        } else {
            let first = mapping.time_signature.bar_n(0);
            let bar_width = mapping.bar_width(first);

            View::Rule {
                index,
                cells: NonZeroU32::MIN,
            }
            .quotated(bar_width)
        }
    })
}
