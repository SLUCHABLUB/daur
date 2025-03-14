//! A simple multi-selection view

use crate::ui::Size;
use crate::view::homogenous::Stack;
use crate::view::{Bordered, Button, Composition, HasSize, OnClick, Text};
use crate::{Cell, ToArcStr};
use arcstr::ArcStr;
use bitbag::{BitBag, Flags};
use crossterm::event::MouseButton;

/// The type returned by [`selector`]
pub type Selector<'cell, T> = Stack<Option<'cell, T>>;

/// A simple multi-selection view
pub fn selector<T: Copy + Flags + ToArcStr + Send + Sync>(cell: &Cell<BitBag<T>>) -> Selector<T>
where
    T::Repr: Send + Sync,
{
    Stack::horizontal_sized(T::VARIANTS.iter().map(move |(_, variant, _)| {
        let name = variant.to_arc_str();

        Option {
            name,
            value: *variant,
            cell,
        }
    }))
    .spacing(1)
}

/// A selection option
#[derive(Debug)]
pub struct Option<'cell, T: Flags> {
    name: ArcStr,
    value: T,
    cell: &'cell Cell<BitBag<T>>,
}

impl<T: Copy + Flags + Send + Sync> Composition for Option<'_, T>
where
    T::Repr: Send + Sync,
{
    type Body<'view>
        = Button<'view, Bordered<Text>>
    where
        Self: 'view;

    fn body(&self) -> Self::Body<'_> {
        let is_set = self.cell.get().is_set(self.value);
        let name = ArcStr::clone(&self.name);

        let on_click = OnClick::new(|button, _, _, _| {
            if button != MouseButton::Left {
                return;
            }

            let mut bag = self.cell.get();

            if bag.is_set(self.value) {
                bag.unset(self.value);
            } else {
                bag.set(self.value);
            }

            self.cell.set(bag);
        });

        Button::standard(name, on_click).border_thickness(is_set)
    }
}

impl<T: Copy + Flags + Send + Sync> HasSize for Option<'_, T>
where
    T::Repr: Send + Sync,
{
    fn size(&self) -> Size {
        self.body().size()
    }
}
