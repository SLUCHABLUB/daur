use crate::app::Action;
use crate::audio::Player;
use crate::metre::Instant;
use crate::project::track::{overview, settings};
use crate::project::{self, ADD_TRACK_DESCRIPTION, ADD_TRACK_LABEL};
use crate::ui::{Grid, Length, NonZeroLength};
use crate::view::{Axis, CursorWindow, OnClick, View, ruler};
use crate::{Project, Selection, UserInterface};

pub(crate) fn workspace<Ui: UserInterface>(
    project: &Project,
    selection: Selection,
    track_settings_width: NonZeroLength,
    negative_overview_offset: Length,
    grid: Grid,
    cursor: Instant,
    player: Option<&Player>,
) -> View {
    let mut track_settings = Vec::new();
    let mut track_overviews = Vec::new();

    for track in project.tracks.values() {
        let selected = selection.track() == track.id();

        track_settings.push(settings(track, selected));
        track_overviews.push(overview(
            track,
            selection,
            project.settings.clone(),
            grid,
            negative_overview_offset,
            cursor,
            player.cloned(),
        ));
    }

    // The "add track" button
    track_settings.push(View::described_button(
        ADD_TRACK_LABEL,
        ADD_TRACK_DESCRIPTION,
        OnClick::from(project::Action::AddTrack),
    ));

    // An empty row (the row with the add-track button)
    track_overviews.push(empty_track_overview(
        project.settings.clone(),
        grid,
        negative_overview_offset,
        cursor,
        player.cloned(),
    ));

    let ruler = ruler::<Ui>(negative_overview_offset, project.settings.clone(), grid);
    let ruler_row = ruler
        .scrollable(Action::MoveOverview)
        .fill_remaining()
        .x_positioned(track_settings_width.get());

    let settings_column = View::balanced_stack(Axis::Y, track_settings);
    let overview_column = View::balanced_stack(Axis::Y, track_overviews);

    let track_area = View::x_stack([
        settings_column.quotated(track_settings_width.get()),
        overview_column.fill_remaining(),
    ]);

    View::y_stack([
        ruler_row.quotated(Ui::RULER_HEIGHT.get()),
        track_area.fill_remaining(),
    ])
}

fn empty_track_overview(
    project_settings: project::Settings,
    grid: Grid,
    negative_overview_offset: Length,
    cursor: Instant,
    player: Option<Player>,
) -> View {
    CursorWindow::builder()
        .cursor(cursor)
        .grid(grid)
        .player(player)
        .project_settings(project_settings)
        .window_offset(negative_overview_offset)
        .build()
        .view()
        .scrollable(Action::MoveOverview)
}
