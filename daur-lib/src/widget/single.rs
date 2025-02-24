//! A simple single-selection widget

use crate::app::Action;
use crate::cell::Cell;
use crate::ui::{Point, Rectangle};
use crate::widget::bordered::Bordered;
use crate::widget::homogenous::Stack;
use crate::widget::injective::Injective;
use crate::widget::text::Text;
use crate::ToArcStr;
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use std::fmt::Display;
use strum::VariantArray;

/// The type returned by [`selector`]
pub type Selector<'cell, T> = Stack<Option<'cell, T>>;

/// A simple single-selection widget
pub fn selector<T: Copy + PartialEq + Display + VariantArray>(cell: &Cell<T>) -> Selector<T> {
    selector_with_formatter(cell, ToArcStr::to_arc_str)
}

/// A simple single-selection widget that uses a custom formatter rather than [`Display`]
pub fn selector_with_formatter<T: Copy + PartialEq + VariantArray, F: FnMut(&T) -> ArcStr>(
    cell: &Cell<T>,
    mut formatter: F,
) -> Selector<T> {
    Stack::horizontal_sized(T::VARIANTS.iter().map(|variant| Option {
        name: formatter(variant),
        value: *variant,
        cell,
    }))
    .spacing(1)
}

/// A selection option
#[derive(Debug)]
pub struct Option<'cell, T: Copy> {
    name: ArcStr,
    value: T,
    cell: &'cell Cell<T>,
}

impl<T: Copy + PartialEq> Injective for Option<'_, T> {
    type Visual = Bordered<Text>;

    fn visual(&self) -> Self::Visual {
        let is_set = self.cell.get() == self.value;
        let name = ArcStr::clone(&self.name);

        Bordered::new(ArcStr::new(), Text::centered(name), is_set)
    }

    fn inject(&self, _: Rectangle, button: MouseButton, _: Point, _: &mut Vec<Action>) {
        if button != MouseButton::Left {
            return;
        }

        self.cell.set(self.value);
    }
}
