use crate::app::action::Action;
use crate::cell::Cell;
use crate::widget::homogenous_stack::HomogenousStack;
use crate::widget::sized::Sized;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect, Size};
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::{Block, Paragraph};
use saturating_cast::SaturatingCast;
use strum::VariantArray;

pub type SingleSelector<'a, T> = HomogenousStack<Option<'a, T>>;

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

pub struct Option<'a, T> {
    name: String,
    value: T,
    cell: &'a Cell<T>,
}

impl<T: Copy + PartialEq> Widget for Option<'_, T> {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let set = if self.cell.get() == self.value {
            THICK
        } else {
            PLAIN
        };

        Paragraph::new(self.name.as_str())
            .block(Block::bordered().border_set(set))
            .centered()
            .render(area, buf, mouse_position);
    }

    fn click(&self, _: Rect, button: MouseButton, _: Position, _: &mut Vec<Action>) {
        if button != MouseButton::Left {
            return;
        }

        self.cell.set(self.value);
    }
}

impl<T: Copy + PartialEq> Sized for Option<'_, T> {
    fn size(&self) -> Size {
        Size {
            width: self.name.chars().count().saturating_cast::<u16>() + 2,
            height: self.name.lines().count().saturating_cast::<u16>() + 2,
        }
    }
}
