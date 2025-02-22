use crate::app::Action;
use crate::cell::Cell;
use crate::ui::{Point, Rectangle};
use crate::widget::bordered::Bordered;
use crate::widget::homogenous::Stack;
use crate::widget::injective::Injective;
use crate::widget::text::Text;
use arcstr::{format, ArcStr};
use crossterm::event::MouseButton;
use std::fmt::Display;
use strum::VariantArray;

pub type Selector<'cell, T> = Stack<Option<'cell, T>>;

pub fn selector<T: Copy + PartialEq + Display + VariantArray>(cell: &Cell<T>) -> Selector<T> {
    Stack::horizontal_sized(T::VARIANTS.iter().map(|variant| {
        let name = format!("{variant}");

        Option {
            name,
            value: *variant,
            cell,
        }
    }))
    .spacing(1)
}

pub struct Option<'cell, T> {
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
