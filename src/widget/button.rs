use crate::app::action::Action;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::length::size::Size;
use crate::length::Length;
use crate::widget::bordered::Bordered;
use crate::widget::has_size::HasSize;
use crate::widget::text::Text;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct Button {
    pub action: Action,
    label: Text,
    description: Option<Text>,
    bordered: bool,
}

impl Button {
    pub fn simple<S: Into<String>>(label: S, action: Action) -> Self {
        Button {
            action,
            label: Text::left_aligned(label),
            description: None,
            bordered: false,
        }
    }

    pub fn standard<S: Into<String>>(label: S, action: Action) -> Bordered<Self> {
        Bordered::plain(
            "",
            Button {
                action,
                label: Text::centered(label),
                description: None,
                bordered: true,
            },
        )
    }

    pub fn described<L: Into<String>, D: Into<String>>(
        label: L,
        description: D,
        action: Action,
    ) -> Bordered<Self> {
        Bordered::plain(
            "",
            Button {
                action,
                label: Text::centered(label),
                description: Some(Text::centered(description)),
                bordered: true,
            },
        )
    }

    fn description(&self) -> &Text {
        self.description.as_ref().unwrap_or(&self.label)
    }
}

// TODO: use injective
impl Widget for Button {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        let text = if area.contains(mouse_position) {
            self.description()
        } else {
            &self.label
        };

        text.render(area, buf, mouse_position);
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
