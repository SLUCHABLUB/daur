use crate::app::Action;
use crate::ui::{Length, Point, Rectangle, Size};
use crate::widget::has_size::HasSize;
use crate::widget::Widget;
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Alignment;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::{Block, WidgetRef as _};

/// A simpler version of [`Block`](widgets::Block)
#[derive(Debug)]
pub struct Bordered<Child> {
    title: ArcStr,
    title_alignment: Alignment,
    thick: bool,
    child: Child,
}

impl<Child> Bordered<Child> {
    pub fn new(title: ArcStr, child: Child, thick: bool) -> Self {
        Bordered {
            title,
            title_alignment: Alignment::Center,
            thick,
            child,
        }
    }

    pub fn plain(title: ArcStr, child: Child) -> Self {
        Bordered::new(title, child, false)
    }

    pub fn thick(title: ArcStr, child: Child) -> Self {
        Bordered::new(title, child, true)
    }

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
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        let block = self.to_block();
        block.render_ref(area.to_rect(), buf);
        self.child.render(self.inner(area), buf, mouse_position);
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
