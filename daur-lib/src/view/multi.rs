//! A simple multi-selection view

use crate::view::{Direction, OnClick, View};
use crate::{Cell, ToArcStr};
use bitbag::{BitBag, Flags};
use std::sync::Arc;

/// A simple multi-selection view
pub fn selector<T: Copy + Flags + ToArcStr + Send + Sync>(
    cell: &Arc<Cell<BitBag<T>>>,
    direction: Direction,
) -> View
where
    T::Repr: Send + Sync,
{
    View::balanced_stack(
        direction,
        T::VARIANTS.iter().map(move |(_, variant, _)| {
            let name = variant.to_arc_str();

            let cell = Arc::clone(cell);
            View::generator(move || {
                let is_set = cell.get().is_set(*variant);

                let cell = Arc::clone(&cell);
                let on_click = OnClick::new(move |_, _, _| {
                    let mut bag = cell.get();

                    if bag.is_set(*variant) {
                        bag.unset(*variant);
                    } else {
                        bag.set(*variant);
                    }

                    cell.set(bag);
                });

                View::standard_button(name.clone(), on_click).with_thickness(is_set)
            })
        }),
    )
}
