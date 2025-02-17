use crate::app::action::Action;
use crate::cell::Cell;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::widget::block::Bordered;
use crate::widget::homogenous_stack::HomogenousStack;
use crate::widget::injective::Injective;
use crate::widget::text::Text;
use bitbag::{BitBag, Flags};
use crossterm::event::MouseButton;

pub type MultiSelector<'cell, T> = HomogenousStack<Option<'cell, T>>;

pub fn multi_selector<T: Copy + Flags + ToString>(cell: &Cell<BitBag<T>>) -> MultiSelector<T> {
    HomogenousStack::horizontal_sized(T::VARIANTS.iter().map(|(_, variant, _)| {
        let name = variant.to_string();

        Option {
            name,
            value: *variant,
            cell,
        }
    }))
    .spacing(1)
}

pub struct Option<'cell, T: Flags> {
    name: String,
    value: T,
    cell: &'cell Cell<BitBag<T>>,
}

impl<T: Copy + Flags> Injective for Option<'_, T> {
    type Visual = Bordered<Text>;

    fn visual(&self) -> Self::Visual {
        let is_set = self.cell.get().is_set(self.value);

        Bordered::new("", Text::centered(&self.name), is_set)
    }

    fn inject(&self, _: Rectangle, button: MouseButton, _: Point, _: &mut Vec<Action>) {
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
    }
}
