mod on_click;

pub use on_click::OnClick;

use crate::app::Action;
use crate::ui::{Point, Rectangle, Size};
use crate::view::has_size::HasSize;
use crate::view::hoverable::Hoverable;
use crate::view::{Bordered, Text, View};
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;

/// A button
#[derive(Debug)]
pub struct Button<'on_click, Content> {
    /// The action to take when the button is clicked
    pub on_click: OnClick<'on_click>,
    /// The default label for the button
    pub content: Content,
}

impl<'on_click> Button<'on_click, Text> {
    /// Constructs a simple button with no border and left aligned text
    #[must_use]
    pub fn simple(label: ArcStr, on_click: OnClick<'on_click>) -> Self {
        Button {
            on_click,
            content: Text::top_left(label),
        }
    }
}

impl<'on_click> Button<'on_click, Bordered<Text>> {
    /// Constructs a standard button with a border and centered text
    #[must_use]
    pub fn standard(label: ArcStr, on_click: OnClick<'on_click>) -> Self {
        Button {
            on_click,
            content: Bordered::plain(Text::centred(label)),
        }
    }

    /// Sets the border thickness.
    #[must_use]
    pub fn border_thickness(mut self, thickness: bool) -> Self {
        self.content.thick = thickness;
        self
    }
}

impl<'on_click> Button<'on_click, Hoverable<Bordered<Text>>> {
    /// Constructs a button with a description, border and centred text
    #[must_use]
    pub fn described(label: ArcStr, description: ArcStr, on_click: OnClick<'on_click>) -> Self {
        Button {
            on_click,
            content: Hoverable {
                default: Bordered::plain(Text::centred(label)),
                hovered: Bordered::plain(Text::centred(description)),
            },
        }
    }
}

impl<Content: View> View for Button<'_, Content> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        self.content.render(area, buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        if button == MouseButton::Left {
            let position = Point::ZERO + (position - area.position);

            self.on_click.run(button, area.size, position, actions);
        }
        self.content.click(area, button, position, actions);
    }
}

impl<Content: HasSize> HasSize for Button<'_, Content> {
    fn size(&self) -> Size {
        self.content.size()
    }
}
