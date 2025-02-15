use crate::widget::to_widget::ToWidget;
use ratatui::layout::Alignment;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets;
use std::borrow::Cow;

/// A simpler version of [`Block`](widgets::Block)
pub struct Block {
    // TODO: maybe use arc_str to avoid cloning?
    title: Cow<'static, str>,
    title_alignment: Alignment,
    thick: bool,
}

impl Block {
    pub fn new(title: impl Into<Cow<'static, str>>, thick: bool) -> Self {
        Block {
            title: title.into(),
            title_alignment: Alignment::Center,
            thick,
        }
    }

    pub fn thick(title: impl Into<Cow<'static, str>>) -> Self {
        Block::new(title, true)
    }
}

impl ToWidget for Block {
    type Widget<'a> = widgets::Block<'static>;

    fn to_widget(&self) -> Self::Widget<'_> {
        widgets::Block::bordered()
            .title(self.title.clone())
            .title_alignment(self.title_alignment)
            .border_set(if self.thick { THICK } else { PLAIN })
    }
}
