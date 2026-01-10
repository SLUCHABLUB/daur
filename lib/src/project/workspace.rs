use crate::Holdable;
use crate::Project;
use crate::UserInterface;
use crate::View;
use crate::app::Action;
use crate::audio::Player;
use crate::metre::Changing;
use crate::metre::Instant;
use crate::metre::OffsetMapping;
use crate::metre::Quantisation;
use crate::metre::TimeContext;
use crate::project;
use crate::project::ADD_TRACK_DESCRIPTION;
use crate::project::ADD_TRACK_LABEL;
use crate::project::track::Overview;
use crate::project::track::clip;
use crate::project::track::settings;
use crate::select::Selection;
use crate::ui;
use crate::ui::Length;
use crate::ui::Size;
use crate::ui::relative;
use crate::view::Axis;
use crate::view::CursorWindow;
use crate::view::OnClick;
use crate::view::ruler;
use non_zero::non_zero;
use std::sync::Arc;

pub(crate) fn workspace<Ui: UserInterface>(
    project: &Project,
    selection: &Selection,
    ui_settings: ui::Settings,
    quantisation: Quantisation,
    cursor: Instant,
    player: Option<&Player>,
    held_object: Option<Holdable>,
) -> View {
    let mut track_settings = Vec::new();
    let mut track_overviews = Vec::new();

    let offset_mapping = OffsetMapping::new(project.time_signature.clone(), quantisation);
    let time_context = project.time_context();

    let held_clip = match held_object {
        Some(Holdable::Clip(id)) => Some(id.clip),
        _ => None,
    };

    for track in project.tracks.values() {
        let selected = selection.contains_track(track.id());

        track_settings.push(settings(track, selected));
        track_overviews.push(
            Overview::builder()
                .track(track)
                .selection(selection)
                .offset_mapping(offset_mapping.clone())
                .time_context(time_context.clone())
                .negative_overview_offset(ui_settings.negative_overview_offset)
                .cursor(cursor)
                .maybe_player(player.cloned())
                .maybe_held_clip(held_clip)
                .build()
                .view(),
        );
    }

    // The "add track" button
    track_settings.push(View::described_button(
        ADD_TRACK_LABEL,
        ADD_TRACK_DESCRIPTION,
        OnClick::from(project::Edit::AddTrack),
    ));

    // An empty row (the row with the add-track button)
    track_overviews.push(empty_track_overview(
        offset_mapping.clone(),
        time_context,
        ui_settings.negative_overview_offset,
        cursor,
        player.cloned(),
    ));

    let ruler = ruler(ui_settings.negative_overview_offset, offset_mapping);
    let ruler_row = ruler
        .scrollable(Action::MoveOverview)
        .fill_remaining()
        .x_positioned(ui_settings.track_settings_width.get());

    let settings_column = View::balanced_stack(Axis::Y, track_settings);
    let overview_column = View::balanced_stack(Axis::Y, track_overviews);

    let overview_column = View::Layers(vec![
        overview_column,
        held_object_view(held_object, project, quantisation).unwrap_or(View::Empty),
    ]);

    let track_area = View::x_stack([
        settings_column.quoted(ui_settings.track_settings_width),
        overview_column.fill_remaining(),
    ]);

    View::y_stack([
        ruler_row.quoted(Ui::RULER_HEIGHT),
        track_area.fill_remaining(),
    ])
}

fn held_object_view(
    held_object: Option<Holdable>,
    project: &Project,
    quantisation: Quantisation,
) -> Option<View> {
    Some(match held_object? {
        Holdable::Clip(path) => {
            let (clip_start, clip) = project.clip(path)?;
            let mapping = OffsetMapping::new(project.time_signature().clone(), quantisation);

            let clip_offset = mapping.offset(clip_start);

            let clip_end = clip_start + clip.duration().get();
            let clip_end_offset = mapping.offset(clip_end);

            let width = clip_end_offset - clip_offset;

            let overview = clip::overview(clip, true, mapping, Length::ZERO, path.track);

            let overview = Arc::new(overview);

            let row_count = non_zero!(1_u64).saturating_add(project.tracks.len() as u64);

            View::reactive(move |render_area| {
                let height = render_area.area.size.height / row_count;

                let position = render_area.saturated_mouse_position();
                let size = Size { width, height };

                View::Shared(Arc::clone(&overview))
                    .quoted_2d(size)
                    .positioned(position)
            })
        }
        Holdable::NoteCreation { .. }
        | Holdable::PianoRollHandle { .. }
        | Holdable::Popup { .. }
        | Holdable::PopupSide { .. } => return None,
        Holdable::SelectionBox { start } => View::reactive(move |render_area| {
            let start = start.relative_to(render_area.area.position);
            let end = render_area.saturated_mouse_position();

            let area = relative::Rectangle::containing_both(start, end);

            View::SelectionBox.positioned(area)
        }),
    })
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
        .maybe_player(player)
        .time_context(time_context)
        .window_offset(negative_overview_offset)
        .build()
        .view()
        .scrollable(Action::MoveOverview)
}
