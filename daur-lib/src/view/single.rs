//! A simple single-selection view

use crate::view::{Direction, OnClick, View};
use crate::{Cell, ToArcStr};
use arcstr::ArcStr;
use std::fmt::Display;
use std::sync::Arc;
use strum::VariantArray;

/// A simple single-selection view
pub fn selector<T: Copy + PartialEq + Display + VariantArray + Send + Sync>(
    cell: &Arc<Cell<T>>,
    direction: Direction,
) -> View {
    selector_with_formatter(cell, direction, ToArcStr::to_arc_str)
}

/// A simple single-selection view that uses a custom formatter rather than [`Display`]
pub fn selector_with_formatter<
    T: Copy + PartialEq + VariantArray + Send + Sync,
    F: FnMut(&T) -> ArcStr,
>(
    cell: &Arc<Cell<T>>,
    direction: Direction,
    mut formatter: F,
) -> View {
    View::balanced_stack(
        direction,
        T::VARIANTS.iter().map(|variant| {
            let name = formatter(variant);

            let cell = Arc::clone(cell);
            View::generator(move || {
                let is_set = cell.get() == *variant;

                let cell = Arc::clone(&cell);
                let on_click = OnClick::new(move |_, _, _| {
                    cell.set(*variant);
                });

                View::standard_button(name.clone(), on_click).with_thickness(is_set)
            })
        }),
    )
}
