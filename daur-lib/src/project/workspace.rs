use crate::app::Selection;
use crate::audio::Player;
use crate::metre::Instant;
use crate::project::{self, ADD_TRACK_DESCRIPTION, ADD_TRACK_LABEL, Settings};
use crate::track::{Track, overview, settings};
use crate::ui::{Grid, Length, NonZeroLength};
use crate::view::{Axis, OnClick, ToText as _, View, ruler};
use crate::{Action, Id, UserInterface};
use arcstr::literal;
use indexmap::map::Values;

// TODO: merge `overview_offset` and `track_settings_width` into temporary settings and remove expect
#[expect(clippy::too_many_arguments, reason = "todo")]
pub(crate) fn workspace<Ui: UserInterface>(
    overview_offset: Length,
    selection: &Selection,
    track_settings_width: NonZeroLength,
    tracks: Values<Id<Track>, Track>,
    project_settings: Settings,
    grid: Grid,
    cursor: Instant,
    player: Option<&Player>,
) -> View {
    let mut track_settings = Vec::new();
    let mut track_overviews = Vec::new();

    for track in tracks {
        let selected = selection.track() == track.id();

        track_settings.push(settings(track, selected));
        track_overviews.push(overview(
            track,
            selection,
            project_settings.clone(),
            grid,
            overview_offset,
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

    // TODO: don't use a dummy here, make a dedicated function
    // A "dummy-track" for the row with the add-track button
    track_overviews.push(overview(
        &Track::new(),
        selection,
        project_settings.clone(),
        grid,
        overview_offset,
        cursor,
        player.cloned(),
    ));

    // TODO: put something here?
    let empty_space = literal!(":)").centred();

    let ruler = ruler::<Ui>(overview_offset, project_settings, grid);
    let ruler_row = View::x_stack([
        empty_space.quotated(track_settings_width.get()),
        ruler.scrollable(Action::MoveOverview).fill_remaining(),
    ]);

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
