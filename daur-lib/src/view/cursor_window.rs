use crate::app::Action;
use crate::audio::Player;
use crate::metre::Instant;
use crate::ui::{Grid, Length};
use crate::view::OnClick;
use crate::{View, project};
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

    project_settings: project::Settings,
    grid: Grid,
}

impl CursorWindow {
    pub(crate) fn view(self) -> View {
        let project_settings = self.project_settings.clone();

        let on_click = OnClick::new(move |render_area, actions| {
            let Some(mouse_position) = render_area.relative_mouse_position() else {
                return;
            };

            let ui_offset = mouse_position.x + self.window_offset;
            let instant = Instant::quantised_from_x_offset(ui_offset, &project_settings, self.grid);

            actions.push(Action::MoveCursor(instant));
        });

        View::CursorWindow(self).on_click(on_click)
    }

    fn player_position(&self) -> Option<Instant> {
        Some(
            self.player
                .as_ref()?
                .position()?
                .to_metre(&self.project_settings),
        )
    }

    /// The cursor's offset from the left of the window.
    /// If `None` is returned, the cursor is not within the window.
    #[must_use]
    pub fn offset(&self) -> Option<Length> {
        let position = self.player_position().unwrap_or(self.cursor);

        let offset = position.to_x_offset(&self.project_settings, self.grid);

        (self.window_offset <= offset).then_some(offset - self.window_offset)
    }
}
