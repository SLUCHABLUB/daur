use crate::project::{ADD_TRACK_DESCRIPTION, ADD_TRACK_LABEL, Action};
use crate::time::Instant;
use crate::track::{Track, overview, settings};
use crate::ui::{Length, Offset};
use crate::view::{Direction, OnClick, ToText as _, View, ruler};
use crate::{time, ui};
use arcstr::literal;
use std::sync::Arc;

// TODO: merge `overview_offset` and `track_settings_width` into temporary settings and remove expect
#[expect(clippy::too_many_arguments, reason = "todo")]
pub(crate) fn workspace(
    overview_offset: Offset,
    selected_track_index: usize,
    selected_clip_index: usize,
    track_settings_width: Length,
    tracks: Vec<Arc<Track>>,
    time_mapping: time::Mapping,
    ui_mapping: ui::Mapping,
    cursor: Instant,
) -> View {
    let mut track_settings = Vec::new();
    let mut track_overviews = Vec::new();

    for (track_index, track) in tracks.into_iter().enumerate() {
        let selected = track_index == selected_track_index;
        track_settings.push(settings(&track, track_index, selected));
        track_overviews.push(overview(
            track,
            track_index,
            selected_clip_index,
            time_mapping.clone(),
            ui_mapping.clone(),
            overview_offset,
            cursor,
        ));
    }

    // The "add track" button
    track_settings.push(View::described_button(
        ADD_TRACK_LABEL,
        ADD_TRACK_DESCRIPTION,
        OnClick::from(Action::AddTrack),
    ));

    // A "dummy-track" for the row with the add-track button
    track_overviews.push(overview(
        Arc::new(Track::new()),
        usize::MAX,
        selected_clip_index,
        time_mapping,
        ui_mapping.clone(),
        overview_offset,
        cursor,
    ));

    // TODO: put something here?
    let empty_space = literal!(":)").centred();

    let ruler = ruler(ui_mapping, overview_offset);
    let ruler_row = View::Stack {
        direction: Direction::Right,
        elements: vec![
            empty_space.quotated(track_settings_width),
            ruler.fill_remaining(),
        ],
    };

    let settings_column = View::balanced_stack(Direction::Down, track_settings);
    let overview_column = View::balanced_stack(Direction::Down, track_overviews);

    let track_area = View::Stack {
        direction: Direction::Right,
        elements: vec![
            settings_column.quotated(track_settings_width),
            overview_column.fill_remaining(),
        ],
    };

    View::Stack {
        direction: Direction::Down,
        elements: vec![ruler_row.quotated_minimally(), track_area.fill_remaining()],
    }
}
