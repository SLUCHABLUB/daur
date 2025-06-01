use crate::Selectable;
use crate::app::Action;
use crate::audio::Player;
use crate::metre::{Changing, Instant, OffsetMapping, TimeContext};
use crate::project::Track;
use crate::project::track::clip;
use crate::select::Selection;
use crate::ui::Length;
use crate::view::context::Menu;
use crate::view::{CursorWindow, View};

// TODO: add a selection box
/// Returns the track overview.
pub(crate) fn overview(
    track: &Track,
    selection: &Selection,
    offset_mapping: OffsetMapping,
    time_context: Changing<TimeContext>,
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
                let absolute_clip_offset = offset_mapping.offset(clip_start);

                let start_crop = negative_overview_offset - absolute_clip_offset;

                let clip_offset = absolute_clip_offset - negative_overview_offset;

                let clip_end = clip_start + clip.duration().get();
                let clip_end_offset = offset_mapping.offset(clip_end) - negative_overview_offset;

                let selected = selection.contains_clip(clip.id());

                let clip_width = clip_end_offset - clip_offset;

                let overview = clip::overview(clip, selected, offset_mapping.clone(), start_crop);

                overview.quotated(clip_width).x_positioned(clip_offset)
            })
            .collect(),
    );

    View::Layers(vec![
        clips,
        CursorWindow::builder()
            .cursor(cursor)
            .offset_mapping(offset_mapping)
            .player(player)
            .time_context(time_context)
            .window_offset(negative_overview_offset)
            .build()
            .view(),
    ])
    .contextual(Menu::track_overview())
    .scrollable(Action::MoveOverview)
    .selectable(Selectable::Track(track.id))
}
