//! A simple single-selection view

use crate::view::{Direction, OnClick, View};
use crate::{Cell, ToArcStr};
use arcstr::ArcStr;
use closure::closure;
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

/// A simple single-selection view that uses a custom "formatter".
pub fn selector_with_formatter<
    T: Copy + PartialEq + VariantArray + Send + Sync,
    F: Fn(&T) -> ArcStr + Clone + Send + Sync + 'static,
>(
    cell: &Arc<Cell<T>>,
    direction: Direction,
    formatter: F,
) -> View {
    View::balanced_stack(
        direction,
        T::VARIANTS.iter().map(|variant| {
            View::generator(closure!([clone cell, clone formatter] move || {
                let is_set = cell.get() == *variant;

                let cell = Arc::clone(&cell);
                let on_click = OnClick::new(move |_, _, _| {
                    cell.set(*variant);
                });

                View::standard_button(formatter(variant), on_click).with_selection_status(is_set)
            }))
        }),
    )
}
