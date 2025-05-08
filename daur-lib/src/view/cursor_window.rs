use crate::audio::Player;
use crate::musical_time::Instant;
use crate::ui::Length;
use crate::view::OnClick;
use crate::{Action, View, musical_time, ui};
use derive_more::Debug;

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
#[derive(Clone, Debug)]
pub struct CursorWindow {
    #[debug(skip)]
    player: Option<Player>,
    cursor: Instant,

    window_offset: Length,

    time_mapping: musical_time::Mapping,
    ui_mapping: ui::Mapping,
}

impl CursorWindow {
    pub(crate) fn view(
        player: Option<Player>,
        cursor: Instant,
        time_mapping: musical_time::Mapping,
        ui_mapping: ui::Mapping,
        window_offset: Length,
    ) -> View {
        let window = CursorWindow {
            player,
            cursor,
            window_offset,
            time_mapping,
            ui_mapping,
        };

        let mapping = window.ui_mapping.clone();

        let on_click = OnClick::new(move |_, position, actions| {
            let ui_offset = position.x + window_offset;
            let instant = mapping.instant_on_grid(ui_offset);

            actions.send(Action::MoveCursor(instant));
        });

        View::CursorWindow(window).on_click(on_click)
    }

    fn player_position(&self) -> Option<Instant> {
        Some(self.time_mapping.musical(self.player.as_ref()?.position()?))
    }

    /// The cursor's offset from the left of the window.
    /// If `None` is returned, the cursor is not within the window.
    #[must_use]
    pub fn offset(&self) -> Option<Length> {
        let position = self.player_position().unwrap_or(self.cursor);

        let offset = self.ui_mapping.x_offset(position);

        (self.window_offset <= offset).then_some(offset - self.window_offset)
    }
}
