//! A simple single-selection view

use crate::ToArcStr;
use crate::sync::Cell;
use crate::view::Axis;
use crate::view::OnClick;
use crate::view::View;
use arcstr::ArcStr;
use enum_iterator::Sequence;
use enum_iterator::all;
use std::fmt::Display;
use std::sync::Arc;

/// A simple single-selection view
pub fn selector<T>(cell: &Arc<Cell<T>>, axis: Axis) -> View
where
    T: Copy + PartialEq + Display + Sequence + Send + Sync + 'static,
{
    selector_with_formatter(cell, axis, ToArcStr::to_arc_str)
}

/// A simple single-selection view that uses a custom "formatter".
pub fn selector_with_formatter<T, F>(cell: &Arc<Cell<T>>, axis: Axis, formatter: F) -> View
where
    T: Copy + PartialEq + Sequence + Send + Sync + 'static,
    F: Fn(&T) -> ArcStr + Clone + Send + Sync + 'static,
{
    View::balanced_stack(
        axis,
        all::<T>().map(|variant| {
            let cell = Arc::clone(cell);
            let formatter = formatter.clone();

            View::reactive(move |_| {
                let is_set = cell.get() == variant;

                let cell = Arc::clone(&cell);
                let on_click = OnClick::new(move |_, _| {
                    cell.set(variant);
                });

                View::toggle(formatter(&variant), on_click, is_set)
            })
        }),
    )
}
