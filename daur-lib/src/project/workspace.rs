use crate::app::Action;
use crate::audio::Player;
use crate::metre::{Changing, Instant, OffsetMapping, Quantisation, TimeContext};
use crate::project::track::{overview, settings};
use crate::project::{self, ADD_TRACK_DESCRIPTION, ADD_TRACK_LABEL};
use crate::ui::Length;
use crate::view::{Axis, CursorWindow, OnClick, View, ruler};
use crate::{Project, Selection, UserInterface, ui};

pub(crate) fn workspace<Ui: UserInterface>(
    project: &Project,
    selection: &Selection,
    ui_settings: ui::Settings,
    quantisation: Quantisation,
    cursor: Instant,
    player: Option<&Player>,
) -> View {
    let mut track_settings = Vec::new();
    let mut track_overviews = Vec::new();

    let offset_mapping = OffsetMapping::new(project.time_signature.clone(), quantisation);
    let time_context = project.time_context();

    for track in project.tracks.values() {
        let selected = selection.track == track.id();

        track_settings.push(settings(track, selected));
        track_overviews.push(overview(
            track,
            selection,
            offset_mapping.clone(),
            time_context.clone(),
            ui_settings.negative_overview_offset,
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
        offset_mapping.clone(),
        time_context,
        ui_settings.negative_overview_offset,
        cursor,
        player.cloned(),
    ));

    let ruler = ruler::<Ui>(ui_settings.negative_overview_offset, offset_mapping);
    let ruler_row = ruler
        .scrollable(Action::MoveOverview)
        .fill_remaining()
        .x_positioned(ui_settings.track_settings_width.get());

    let settings_column = View::balanced_stack(Axis::Y, track_settings);
    let overview_column = View::balanced_stack(Axis::Y, track_overviews);

    let track_area = View::x_stack([
        settings_column.quotated(ui_settings.track_settings_width.get()),
        overview_column.fill_remaining(),
    ]);

    View::y_stack([
        ruler_row.quotated(Ui::RULER_HEIGHT.get()),
        track_area.fill_remaining(),
    ])
}

fn empty_track_overview(
    offset_mapping: OffsetMapping,
    time_context: Changing<TimeContext>,
    negative_overview_offset: Length,
    cursor: Instant,
    player: Option<Player>,
) -> View {
    CursorWindow::builder()
        .cursor(cursor)
        .offset_mapping(offset_mapping)
        .player(player)
        .time_context(time_context)
        .window_offset(negative_overview_offset)
        .build()
        .view()
        .scrollable(Action::MoveOverview)
}
