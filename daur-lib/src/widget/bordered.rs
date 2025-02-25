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
pub struct Bordered<Child> {
    title: ArcStr,
    title_alignment: Alignment,
    thick: bool,
    child: Child,
}

impl<Child> Bordered<Child> {
    /// Creates a titled border around `child`
    pub fn new(title: ArcStr, child: Child, thick: bool) -> Self {
        Bordered {
            title,
            title_alignment: Alignment::Center,
            thick,
            child,
        }
    }

    /// Creates a titled plain border around `child`
    pub fn plain(title: ArcStr, child: Child) -> Self {
        Bordered::new(title, child, false)
    }

    /// Creates a titled **thick** border around `child`
    pub fn thick(title: ArcStr, child: Child) -> Self {
        Bordered::new(title, child, true)
    }

    /// Sets whether `self` is **thick**
    #[must_use]
    pub fn thickness(mut self, thick: bool) -> Self {
        self.thick = thick;
        self
    }

    fn to_block(&self) -> Block {
        Block::bordered()
            .title(&*self.title)
            .title_alignment(self.title_alignment)
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
        self.child.render(self.inner(area), buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        self.child
            .click(self.inner(area), button, position, actions);
    }
}

impl<Child: HasSize> HasSize for Bordered<Child> {
    fn size(&self) -> Size {
        let mut size = self.child.size();
        size.height += Length::DOUBLE_BORDER;
        size.width += Length::DOUBLE_BORDER;
        size
    }
}
