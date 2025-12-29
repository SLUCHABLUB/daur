use crate::View;
use crate::app::Action;
use crate::audio::Player;
use crate::metre::Changing;
use crate::metre::Instant;
use crate::metre::OffsetMapping;
use crate::metre::TimeContext;
use crate::ui::Length;
use crate::view::OnClick;
use derive_more::Debug;
use typed_builder::TypedBuilder;

//       |---o---|
//
// 0             | <- cursor
// |     |-------|------------|
// |     |       |            | <- cursor window
// |     |       |            |
// |     |-------|------------|
//               |
// |--w--|
// |------x------|
//
// x: Mapping::x_offset
// o: CursorWindow::offset
// w: CursorWindow::window_offset

/// A window with a musical cursor.
#[derive(Clone, Debug, TypedBuilder)]
#[builder(builder_type(vis = "pub(crate)"))]
pub struct CursorWindow {
    #[debug(skip)]
    player: Option<Player>,
    cursor: Instant,

    window_offset: Length,

    offset_mapping: OffsetMapping,
    time_context: Changing<TimeContext>,
}

impl CursorWindow {
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
