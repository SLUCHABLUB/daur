//! A simple single-selection view

use crate::sync::Cell;
use crate::view::{Axis, OnClick, View};
use crate::{ToArcStr, UserInterface};
use alloc::sync::Arc;
use arcstr::ArcStr;
use closure::closure;
use core::fmt::Display;
use strum::VariantArray;

/// A simple single-selection view
pub fn selector<Ui: UserInterface, T: Copy + PartialEq + Display + VariantArray + Send + Sync>(
    cell: &Arc<Cell<T>>,
    axis: Axis,
) -> View {
    selector_with_formatter::<Ui, _, _>(cell, axis, ToArcStr::to_arc_str)
}

/// A simple single-selection view that uses a custom "formatter".
pub fn selector_with_formatter<
    Ui: UserInterface,
    T: Copy + PartialEq + VariantArray + Send + Sync,
    F: Fn(&T) -> ArcStr + Clone + Send + Sync + 'static,
>(
    cell: &Arc<Cell<T>>,
    axis: Axis,
    formatter: F,
) -> View {
    View::balanced_stack::<Ui, _>(
        axis,
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
