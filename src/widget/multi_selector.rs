use crate::app::action::Action;
use crate::cell::Cell;
use crate::widget::homogenous_stack::HomogenousStack;
use crate::widget::sized::Sized;
use crate::widget::Widget;
use bitbag::{BitBag, Flags};
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect, Size};
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::{Block, Paragraph};
use saturating_cast::SaturatingCast;

pub type MultiSelector<'a, T> = HomogenousStack<Option<'a, T>>;

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

pub struct Option<'a, T: Flags> {
    name: String,
    value: T,
    cell: &'a Cell<BitBag<T>>,
}

impl<T: Copy + Flags> Widget for Option<'_, T> {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let set = if self.cell.get().is_set(self.value) {
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

        let mut bag = self.cell.get();

        if bag.is_set(self.value) {
            bag.unset(self.value);
        } else {
            bag.set(self.value);
        }

        self.cell.set(bag);
    }
}

impl<T: Copy + Flags> Sized for Option<'_, T> {
    fn size(&self) -> Size {
        Size {
            width: self.name.chars().count().saturating_cast::<u16>() + 2,
            height: self.name.lines().count().saturating_cast::<u16>() + 2,
        }
    }
}
