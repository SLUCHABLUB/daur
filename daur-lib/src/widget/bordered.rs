use crate::app::Action;
use crate::ui::{Length, Point, Rectangle, Size};
use crate::widget::has_size::HasSize;
use crate::widget::Widget;
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Alignment;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets;
use ratatui::widgets::Block;

/// A simpler version of [`Block`](widgets::Block)
#[derive(Debug)]
pub struct Bordered<Content> {
    /// The title to display on the borders top edge.
    pub title: ArcStr,
    /// Whether the border is **thick**.
    pub thick: bool,
    /// The content within the border.
    pub content: Content,
}

impl<Content> Bordered<Content> {
    /// Creates a border around `content` without a title
    pub fn plain(content: Content) -> Self {
        Bordered {
            title: ArcStr::new(),
            thick: false,
            content,
        }
    }

    /// Creates a border around `content` with a title
    pub fn titled(title: ArcStr, content: Content) -> Self {
        Bordered {
            title,
            thick: false,
            content,
        }
    }

    /// Sets whether `self` is **thick**.
    #[must_use]
    pub fn thickness(mut self, thick: bool) -> Self {
        self.thick = thick;
        self
    }

    fn to_block(&self) -> Block {
        Block::bordered()
            .title(&*self.title)
            .title_alignment(Alignment::Center)
            .border_set(if self.thick { THICK } else { PLAIN })
    }

    fn inner(&self, area: Rectangle) -> Rectangle {
        Rectangle::from_rect(self.to_block().inner(area.to_rect()))
    }
}

impl<Child: Widget> Widget for Bordered<Child> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let block = self.to_block();
        widgets::Widget::render(block, area.to_rect(), buffer);
        self.content
            .render(self.inner(area), buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        self.content
            .click(self.inner(area), button, position, actions);
    }
}

impl<Child: HasSize> HasSize for Bordered<Child> {
    fn size(&self) -> Size {
        let mut size = self.content.size();
        size.height += Length::DOUBLE_BORDER;
        size.width += Length::DOUBLE_BORDER;
        size
    }
}
