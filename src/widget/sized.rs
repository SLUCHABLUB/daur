use crate::lock::Lock;
use crate::widget::Widget;
use ratatui::layout::{Direction, Size};
use ratatui_explorer::FileExplorer;
use saturating_cast::SaturatingCast;

pub trait Sized: Widget {
    fn size(&self) -> Size;
}

impl<T: Sized> Sized for &T {
    fn size(&self) -> Size {
        (*self).size()
    }
}

/// Splits a `size` into a dominant and subdominant coordinate based on a direction
pub(super) fn split(size: Size, direction: Direction) -> [u16; 2] {
    match direction {
        Direction::Horizontal => [size.width, size.height],
        Direction::Vertical => [size.height, size.width],
    }
}

/// Un`split` a `Size`
pub(super) fn join(dominant: u16, non_dominant: u16, direction: Direction) -> Size {
    match direction {
        Direction::Horizontal => Size::new(dominant, non_dominant),
        Direction::Vertical => Size::new(non_dominant, dominant),
    }
}

// This implementation breaks with ome non-default themes
// but it sufficient for out purposes at the moment
impl Sized for Lock<FileExplorer> {
    fn size(&self) -> Size {
        let explorer = self.read();

        let mut height = explorer.files().len().saturating_cast();

        // This assumes that we use the default title
        let title_width = explorer.cwd().to_string_lossy().chars().count();
        let content_width = explorer
            .files()
            .iter()
            .map(|file| file.name().chars().count())
            .max()
            .unwrap_or(0);

        let mut width = usize::max(title_width, content_width).saturating_cast();

        // This assumes that the block is fully bordered
        // which we can't check since `ratatui` doesn't expose that field
        if explorer.theme().block().is_some() {
            height += 2;
            width += 2;
        }

        Size { width, height }
    }
}
