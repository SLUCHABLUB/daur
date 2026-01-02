//! A simple multi-selection view

use crate::ToArcStr;
use crate::sync::Cell;
use crate::view::Axis;
use crate::view::OnClick;
use crate::view::View;
use enumset::EnumSet;
use enumset::EnumSetType;
use std::sync::Arc;
use strum::VariantArray;

/// A simple multi-selection view.
pub fn selector<T>(cell: &Arc<Cell<EnumSet<T>>>, axis: Axis) -> View
where
    T: Copy + EnumSetType + ToArcStr + Send + Sync + VariantArray,
{
    View::balanced_stack(
        axis,
        T::VARIANTS.iter().map(move |variant| {
            let name = variant.to_arc_str();

            let cell = Arc::clone(cell);
            View::reactive(move |_| {
                let is_set = cell.get().contains(*variant);

                let cell = Arc::clone(&cell);
                let on_click = OnClick::new(move |_, _| {
                    let mut set = cell.get();

                    set ^= *variant;

                    cell.set(set);
                });

                View::toggle(name.clone(), on_click, is_set)
            })
        }),
    )
}
