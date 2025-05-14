use crate::audio::Player;
use crate::metre::Instant;
use crate::project::Settings;
use crate::ui::{Grid, Length};
use crate::view::OnClick;
use crate::{Action, View};
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

    project: Settings,
    grid: Grid,
}

impl CursorWindow {
    // TODO: make this a method
    pub(crate) fn view(
        player: Option<Player>,
        cursor: Instant,
        project: Settings,
        grid: Grid,
        window_offset: Length,
    ) -> View {
        let settings = project.clone();

        let window = CursorWindow {
            player,
            cursor,
            window_offset,
            project,
            grid,
        };

        let on_click = OnClick::new(move |_, position, actions| {
            let ui_offset = position.x + window_offset;
            let instant = Instant::quantised_from_x_offset(ui_offset, &settings, grid);

            actions.push(Action::MoveCursor(instant));
        });

        View::CursorWindow(window).on_click(on_click)
    }

    fn player_position(&self) -> Option<Instant> {
        Some(self.player.as_ref()?.position()?.to_metre(&self.project))
    }

    /// The cursor's offset from the left of the window.
    /// If `None` is returned, the cursor is not within the window.
    #[must_use]
    pub fn offset(&self) -> Option<Length> {
        let position = self.player_position().unwrap_or(self.cursor);

        let offset = position.to_x_offset(&self.project, self.grid);

        (self.window_offset <= offset).then_some(offset - self.window_offset)
    }
}
