use crate::project::{ADD_TRACK_DESCRIPTION, ADD_TRACK_LABEL, Action};
use crate::time::Instant;
use crate::track::{Track, overview, settings};
use crate::ui::{NonZeroLength, Offset};
use crate::view::{Axis, OnClick, ToText as _, View, ruler};
use crate::{Clip, UserInterface, time, ui};
use arcstr::literal;
use std::sync::{Arc, Weak};

// TODO: merge `overview_offset` and `track_settings_width` into temporary settings and remove expect
#[expect(clippy::too_many_arguments, reason = "todo")]
pub(crate) fn workspace<Ui: UserInterface>(
    overview_offset: Offset,
    selected_track: &Weak<Track>,
    selected_clip: &Weak<Clip>,
    track_settings_width: NonZeroLength,
    tracks: Vec<Arc<Track>>,
    time_mapping: time::Mapping,
    ui_mapping: ui::Mapping,
    cursor: Instant,
) -> View {
    let mut track_settings = Vec::new();
    let mut track_overviews = Vec::new();

    for track in tracks {
        let track_reference = Arc::downgrade(&track);
        let selected = selected_track.as_ptr() == track_reference.as_ptr();

        track_settings.push(settings(&track, selected));
        track_overviews.push(overview(
            track,
            selected_clip,
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
        selected_clip,
        time_mapping,
        ui_mapping.clone(),
        overview_offset,
        cursor,
    ));

    // TODO: put something here?
    let empty_space = literal!(":)").centred();

    let ruler = ruler(ui_mapping, overview_offset);
    let ruler_row = View::x_stack([
        empty_space.quotated(track_settings_width.get()),
        ruler.fill_remaining(),
    ]);

    let settings_column = View::balanced_stack::<Ui, _>(Axis::Y, track_settings);
    let overview_column = View::balanced_stack::<Ui, _>(Axis::Y, track_overviews);

    let track_area = View::x_stack([
        settings_column.quotated(track_settings_width.get()),
        overview_column.fill_remaining(),
    ]);

    View::y_stack([
        ruler_row.quotated(Ui::RULER_HEIGHT.get()),
        track_area.fill_remaining(),
    ])
}
