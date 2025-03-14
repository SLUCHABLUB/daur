use crate::lock::Lock;
use crate::ui::{Length, Size};
use crate::view::View;
use ratatui_explorer::FileExplorer;

/// A view with a _"canonical"_ size
pub trait HasSize: View {
    /// Returns the view's _"canonical"_ size
    fn size(&self) -> Size;
}

// This implementation breaks with ome non-default themes
// but it sufficient for out purposes at the moment
impl HasSize for Lock<FileExplorer> {
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
