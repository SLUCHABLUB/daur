use crate::ui::{Length, Mapping, Offset};
use crate::view::{Direction, View, feed};
use std::num::NonZeroU64;

/// A ruler of musical time
pub fn ruler(mapping: Mapping, offset: Length) -> View {
    // TODO: don't use feed
    feed(Direction::Right, Offset::negative(offset), move |index| {
        if let Ok(bar_index) = usize::try_from(index) {
            let bar = mapping.time_signature.bar_n(bar_index);
            let bar_width = mapping.bar_width(bar);
            let cell_width = mapping.grid.cell_width;

            let cells = (bar_width / cell_width).ceil();
            let Some(cells) = NonZeroU64::new(cells) else {
                return View::Empty.quotated(bar_width);
            };

            View::Rule { index, cells }.quotated(bar_width)
        } else {
            let first = mapping.time_signature.bar_n(0);
            let bar_width = mapping.bar_width(first);

            View::Rule {
                index,
                cells: NonZeroU64::MIN,
            }
            .quotated(bar_width)
        }
    })
}
