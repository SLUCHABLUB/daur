use crate::length::size::Size;
use crate::length::Length;
use crate::lock::Lock;
use crate::widget::Widget;
use ratatui::layout::Direction;
use ratatui_explorer::FileExplorer;

pub trait HasSize: Widget {
    fn size(&self) -> Size;
}

/// Splits a `size` into a dominant and subdominant coordinate based on a direction
pub(super) fn split(size: Size, direction: Direction) -> [Length; 2] {
    match direction {
        Direction::Horizontal => [size.width, size.height],
        Direction::Vertical => [size.height, size.width],
    }
}

/// Un`split` a `Size`
pub(super) fn join(dominant: Length, non_dominant: Length, direction: Direction) -> Size {
    match direction {
        Direction::Horizontal => Size {
            width: dominant,
            height: non_dominant,
        },
        Direction::Vertical => Size {
            width: non_dominant,
            height: dominant,
        },
    }
}

// This implementation breaks with ome non-default themes
// but it sufficient for out purposes at the moment
impl HasSize for &Lock<FileExplorer> {
    fn size(&self) -> Size {
        let explorer = self.read();

        let mut height = Length::CHAR_HEIGHT * explorer.files().len();

        // This assumes that we use the default title
        let title_width = Length::string_width(&explorer.cwd().display().to_string());
        let content_width = explorer
            .files()
            .iter()
            .map(|file| Length::string_width(file.name()))
            .max()
            .unwrap_or(Length::ZERO);

        let mut width = Length::max(title_width, content_width);

        // This assumes that the block is fully bordered
        // which we can't check since `ratatui` doesn't expose that field
        if explorer.theme().block().is_some() {
            height += Length::DOUBLE_BORDER;
            width += Length::DOUBLE_BORDER;
        }

        Size { width, height }
    }
}
