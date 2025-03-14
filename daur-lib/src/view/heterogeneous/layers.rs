use crate::app::Action;
use crate::ui::{Point, Rectangle};
use crate::view::View;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;

/// Layers of heterogeneous views
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Layers<Children> {
    /// The layered views
    pub children: Children,
}

impl<Children> Layers<Children> {
    /// Constructs a new `Layers`
    #[must_use]
    pub fn new(children: Children) -> Layers<Children> {
        Layers { children }
    }
}

macro_rules! impl_layers {
    ($len:literal; $($generic:ident),*; $($index:tt),*) => {
        impl<$($generic: View),*> View for Layers<($($generic),*)> {
            fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
                $(
                    self.children.$index.render(area, buffer, mouse_position);
                )*
            }

            fn click(&self, area: Rectangle, button: MouseButton, position: Point, actions: &mut Vec<Action>) {
                $(
                    self.children.$index.click(area, button, position, actions);
                )*
            }
        }
    };
}

impl_layers!(2; A, B; 0, 1);
