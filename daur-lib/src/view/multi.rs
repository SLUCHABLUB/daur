//! A simple multi-selection view

use crate::ToArcStr;
use crate::sync::Cell;
use crate::view::Axis;
use crate::view::OnClick;
use crate::view::View;
use bitbag::BitBag;
use bitbag::Flags;
use std::sync::Arc;

/// A simple multi-selection view
pub fn selector<T: Copy + Flags + ToArcStr + Send + Sync>(
    cell: &Arc<Cell<BitBag<T>>>,
    axis: Axis,
) -> View
where
    T::Repr: Send + Sync,
{
    View::balanced_stack(
        axis,
        T::VARIANTS.iter().map(move |(_, variant, _)| {
            let name = variant.to_arc_str();

            let cell = Arc::clone(cell);
            View::reactive(move |_| {
                let is_set = cell.get().is_set(*variant);

                let cell = Arc::clone(&cell);
                let on_click = OnClick::new(move |_, _| {
                    let mut bag = cell.get();

                    if bag.is_set(*variant) {
                        bag.unset(*variant);
                    } else {
                        bag.set(*variant);
                    }

                    cell.set(bag);
                });

                View::toggle(name.clone(), on_click, is_set)
            })
        }),
    )
}
