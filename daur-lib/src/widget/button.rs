use crate::app::Action;
use crate::ui::{Length, Point, Rectangle, Size};
use crate::widget::bordered::Bordered;
use crate::widget::has_size::HasSize;
use crate::widget::text::Text;
use crate::widget::Widget;
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;

/// A button
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Button {
    /// The action to take when the button is clicked
    pub action: Action,
    /// The default label for the button
    pub label: Text,
    /// An optional description to display when the button is hovered
    pub description: Option<Text>,
}

impl Button {
    /// Constructs a simple button with no border and left aligned text
    #[must_use]
    pub fn simple(label: ArcStr, action: Action) -> Self {
        Button {
            action,
            label: Text::left_aligned(label),
            description: None,
        }
    }

    /// Constructs a standard button with a border and centered text
    #[must_use]
    pub fn standard(label: ArcStr, action: Action) -> Bordered<Self> {
        Bordered::plain(
            ArcStr::new(),
            Button {
                action,
                label: Text::centered(label),
                description: None,
            },
        )
    }

    /// Constructs a button with a description, border and centered text
    #[must_use]
    pub fn described(label: ArcStr, description: ArcStr, action: Action) -> Bordered<Self> {
        Bordered::plain(
            ArcStr::new(),
            Button {
                action,
                label: Text::centered(label),
                description: Some(Text::centered(description)),
            },
        )
    }

    fn description(&self) -> &Text {
        self.description.as_ref().unwrap_or(&self.label)
    }
}

// TODO: use injective
impl Widget for Button {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let text = if area.contains(mouse_position) {
            self.description()
        } else {
            &self.label
        };

        text.render(area, buffer, mouse_position);
    }

    fn click(&self, _: Rectangle, button: MouseButton, _: Point, actions: &mut Vec<Action>) {
        if button != MouseButton::Left {
            return;
        }

        actions.push(self.action.clone());
    }
}

impl HasSize for Button {
    fn size(&self) -> Size {
        let label = self.label.size();
        let description = self.description().size();

        Size {
            width: Length::max(label.width, description.width),
            height: Length::max(label.height, description.height),
        }
    }
}
