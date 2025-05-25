//! A simple single-selection view

use crate::ToArcStr;
use crate::sync::Cell;
use crate::view::{Axis, OnClick, View};
use arcstr::ArcStr;
use closure::closure;
use std::fmt::Display;
use std::sync::Arc;
use strum::VariantArray;

/// A simple single-selection view
pub fn selector<T: Copy + PartialEq + Display + VariantArray + Send + Sync>(
    cell: &Arc<Cell<T>>,
    axis: Axis,
) -> View {
    selector_with_formatter(cell, axis, ToArcStr::to_arc_str)
}

/// A simple single-selection view that uses a custom "formatter".
pub fn selector_with_formatter<
    T: Copy + PartialEq + VariantArray + Send + Sync,
    F: Fn(&T) -> ArcStr + Clone + Send + Sync + 'static,
>(
    cell: &Arc<Cell<T>>,
    axis: Axis,
    formatter: F,
) -> View {
    View::balanced_stack(
        axis,
        T::VARIANTS.iter().map(|variant| {
            View::reactive(closure!([clone cell, clone formatter] move |_| {
                let is_set = cell.get() == *variant;

                let cell = Arc::clone(&cell);
                let on_click = OnClick::new(move |_, _| {
                    cell.set(*variant);
                });

                View::standard_button(formatter(variant), on_click).with_selection_status(is_set)
            }))
        }),
    )
}
