//! A simple single-selection widget

use crate::cell::Cell;
use crate::ui::Size;
use crate::widget::bordered::Bordered;
use crate::widget::homogenous::Stack;
use crate::widget::text::Text;
use crate::widget::{Button, HasSize, OnClick, ToWidget};
use crate::ToArcStr;
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use std::fmt::Display;
use strum::VariantArray;

/// The type returned by [`selector`]
pub type Selector<'cell, T> = Stack<Option<'cell, T>>;

/// A simple single-selection widget
pub fn selector<T: Copy + PartialEq + Display + VariantArray + Send + Sync>(
    cell: &Cell<T>,
) -> Selector<T> {
    selector_with_formatter(cell, ToArcStr::to_arc_str)
}

/// A simple single-selection widget that uses a custom formatter rather than [`Display`]
pub fn selector_with_formatter<
    T: Copy + PartialEq + VariantArray + Send + Sync,
    F: FnMut(&T) -> ArcStr,
>(
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

impl<T: Copy + PartialEq + Send + Sync> ToWidget for Option<'_, T> {
    type Widget<'widget>
        = Button<'widget, Bordered<Text>>
    where
        Self: 'widget;

    fn to_widget(&self) -> Self::Widget<'_> {
        let is_set = self.cell.get() == self.value;
        let name = ArcStr::clone(&self.name);

        let on_click = OnClick::new(|button, _, _, _| {
            if button != MouseButton::Left {
                return;
            }

            self.cell.set(self.value);
        });

        Button {
            on_click,
            content: Bordered::plain(Text::centred(name)).thickness(is_set),
        }
    }
}

impl<T: Copy + PartialEq + Send + Sync> HasSize for Option<'_, T> {
    fn size(&self) -> Size {
        self.to_widget().size()
    }
}
