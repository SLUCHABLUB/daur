//! Items pertaining to [`CursorWindow`].

use crate::View;
use crate::app::Action;
use crate::audio::Player;
use crate::metre::Changing;
use crate::metre::Instant;
use crate::metre::OffsetMapping;
use crate::metre::TimeContext;
use crate::ui::Length;
use crate::view::OnClick;
use bon::Builder;
use derive_more::Debug;

/// A window with a musical cursor.
///
/// See the below illustration for an explanation of the fields:
///
/// ```text
///       |---o---|
///
/// 0             | <- cursor
/// |     |-------|------------|
/// |     |       |            | <- cursor window
/// |     |       |            |
/// |     |-------|------------|
///               |
// TODO: Use `cfg_attr` to only add this if we are documenting  private items.
/// |--w--|
/// |------x------|
/// ```
///
/// Where:
/// - `o`: [`CursorWindow::offset`]
/// - `x`: [`OffsetMapping::offset`]
// TODO: Use `cfg_attr` to only add this if we are documenting  private items.
#[expect(rustdoc::private_intra_doc_links, reason = "TODO")]
/// - `w`: [`CursorWindow::window_offset`]
#[derive(Clone, Debug, Builder)]
pub struct CursorWindow {
    // The setter needs to be `pub(crate)` since `Player` is only `pub(crate)`.
    /// The audio player, used to get the cursor position.
    #[builder(setters(vis = "pub(crate)"))]
    #[debug(skip)]
    player: Option<Player>,
    /// The last position of the cursor.
    /// This will be used if the audio player does not report a position.
    cursor: Instant,

    /// How far the window is offset from the "origin".
    window_offset: Length,

    /// The mapping between time and position.
    offset_mapping: OffsetMapping,
    /// The time context of the windowed track.
    time_context: Changing<TimeContext>,
}

impl CursorWindow {
    /// The clickable view of the cursor window.
    pub(crate) fn view(self) -> View {
        let offset_mapping = self.offset_mapping.clone();

        let on_click = OnClick::new(move |render_area, actions| {
            let Some(mouse_position) = render_area.relative_mouse_position() else {
                return;
            };

            let ui_offset = mouse_position.x + self.window_offset;
            let instant = offset_mapping.quantised_instant(ui_offset);

            actions.push(Action::MoveCursor(instant));
        });

        View::CursorWindow(self).on_click(on_click)
    }

    /// Returns the position of the cursor.
    fn player_position(&self) -> Option<Instant> {
        Some(self.player.as_ref()?.position()? / &self.time_context)
    }

    /// The cursor's offset from the left of the window.
    /// If `None` is returned, the cursor is not within the window.
    #[must_use]
    pub fn offset(&self) -> Option<Length> {
        let position = self.player_position().unwrap_or(self.cursor);

        let offset = self.offset_mapping.offset(position);

        (self.window_offset <= offset).then_some(offset - self.window_offset)
    }
}
