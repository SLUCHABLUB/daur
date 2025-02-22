use crate::app::Action;
use crate::measure::{Length, Point, Rectangle, Size};
use crate::widget::bordered::Bordered;
use crate::widget::has_size::HasSize;
use crate::widget::text::Text;
use crate::widget::Widget;
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Button {
    pub action: Action,
    label: Text,
    description: Option<Text>,
    bordered: bool,
}

impl Button {
    pub fn simple(label: ArcStr, action: Action) -> Self {
        Button {
            action,
            label: Text::left_aligned(label),
            description: None,
            bordered: false,
        }
    }

    pub fn standard(label: ArcStr, action: Action) -> Bordered<Self> {
        Bordered::plain(
            ArcStr::new(),
            Button {
                action,
                label: Text::centered(label),
                description: None,
                bordered: true,
            },
        )
    }

    pub fn described(label: ArcStr, description: ArcStr, action: Action) -> Bordered<Self> {
        Bordered::plain(
            ArcStr::new(),
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
