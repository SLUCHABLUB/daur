use crate::app::Action;
use crate::audio::Player;
use crate::metre::{Changing, Instant, OffsetMapping, TimeContext};
use crate::project::track::clip;
use crate::project::track::clip::Path;
use crate::project::{Edit, Track};
use crate::select::Selection;
use crate::ui::Length;
use crate::view::context::Menu;
use crate::view::{CursorWindow, RenderArea, View};
use crate::{Holdable, Id, Selectable};

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

                let selected = selection.contains_clip(Path::new(track.id, clip.id()));

                let clip_width = clip_end_offset - clip_offset;

                let overview =
                    clip::overview(clip, selected, offset_mapping.clone(), start_crop, track.id);

                overview.quotated(clip_width).x_positioned(clip_offset)
            })
            .collect(),
    );

    let background = View::Empty
        .contextual(Menu::track_overview())
        .selectable(Selectable::Track(track.id))
        .object_accepting(object_acceptor(
            track.id,
            offset_mapping.clone(),
            negative_overview_offset,
        ));

    View::Layers(vec![
        background,
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
    .scrollable(Action::MoveOverview)
}

fn object_acceptor(
    track: Id<Track>,
    offset_mapping: OffsetMapping,
    negative_overview_offset: Length,
) -> impl Fn(Holdable, RenderArea) -> Option<Action> {
    move |holdable, render_area| {
        let Holdable::Clip(clip) = holdable else {
            return None;
        };

        let mouse = render_area.relative_mouse_position()?;

        let position = offset_mapping.quantised_instant(mouse.x + negative_overview_offset);

        Some(Action::Edit(Edit::MoveClip {
            clip,
            track,
            position,
        }))
    }
}
