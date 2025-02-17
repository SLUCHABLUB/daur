use crate::app::action::Action;
use crate::cell::Cell;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::widget::bordered::Bordered;
use crate::widget::homogenous_stack::HomogenousStack;
use crate::widget::injective::Injective;
use crate::widget::text::Text;
use crossterm::event::MouseButton;
use strum::VariantArray;

pub type SingleSelector<'cell, T> = HomogenousStack<Option<'cell, T>>;

pub fn single_selector<T: Copy + PartialEq + ToString + VariantArray>(
    cell: &Cell<T>,
) -> SingleSelector<T> {
    HomogenousStack::horizontal_sized(T::VARIANTS.iter().map(|variant| {
        let name = variant.to_string();

        Option {
            name,
            value: *variant,
            cell,
        }
    }))
    .spacing(1)
}

pub struct Option<'cell, T> {
    name: String,
    value: T,
    cell: &'cell Cell<T>,
}

impl<T: Copy + PartialEq> Injective for Option<'_, T> {
    type Visual = Bordered<Text>;

    fn visual(&self) -> Self::Visual {
        let is_set = self.cell.get() == self.value;
        Bordered::new("", Text::centered(&self.name), is_set)
    }

    fn inject(&self, _: Rectangle, button: MouseButton, _: Point, _: &mut Vec<Action>) {
        if button != MouseButton::Left {
            return;
        }

        self.cell.set(self.value);
    }
}
