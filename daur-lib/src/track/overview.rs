use crate::app::{Action, Selection};
use crate::audio::Player;
use crate::metre::Instant;
use crate::project::Settings;
use crate::track::clip;
use crate::ui::{Grid, Length};
use crate::view::context::Menu;
use crate::view::{CursorWindow, OnClick, View};
use crate::{Id, Track};

/// Returns the track overview.
pub(crate) fn overview(
    track: &Track,
    selection: &Selection,
    project: Settings,
    grid: Grid,
    negative_overview_offset: Length,
    cursor: Instant,
    player: Option<Player>,
) -> View {
    let clips = View::Layers(
        track
            .clips
            .values()
            .map(|clip| {
                let clip_start = track
                    .clip_starts
                    .get(&clip.id())
                    .copied()
                    .unwrap_or_default();
                let absolute_clip_offset = clip_start.to_x_offset(&project, grid);

                let start_crop = negative_overview_offset - absolute_clip_offset;

                let clip_offset = absolute_clip_offset - negative_overview_offset;

                let clip_end = clip.period(clip_start, &project).get().end();
                let clip_end_offset =
                    clip_end.to_x_offset(&project, grid) - negative_overview_offset;

                let selected = selection.clip() == clip.id();

                let clip_width = clip_end_offset - clip_offset;

                let overview = clip::overview(clip, track.id, selected, &project, grid, start_crop);

                View::x_stack([
                    View::Empty.quotated(clip_offset),
                    overview.quotated(clip_width),
                    View::Empty.fill_remaining(),
                ])
            })
            .collect(),
    );

    View::Layers(vec![
        clips,
        CursorWindow::view(player, cursor, project, grid, negative_overview_offset),
    ])
    .on_click(OnClick::from(Action::SelectClip {
        track: track.id,
        clip: Id::NONE,
    }))
    .contextual(Menu::track_overview())
    .scrollable(Action::MoveOverview)
}
