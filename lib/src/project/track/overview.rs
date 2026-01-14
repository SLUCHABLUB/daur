//! Items pertaining to [`Overview`].

use crate::Holdable;
use crate::Id;
use crate::Selectable;
use crate::View;
use crate::app::Action;
use crate::audio::Player;
use crate::metre::Changing;
use crate::metre::Instant;
use crate::metre::OffsetMapping;
use crate::metre::TimeContext;
use crate::project::Edit;
use crate::project::Track;
use crate::project::track::Clip;
use crate::project::track::clip;
use crate::project::track::clip::Path;
use crate::select::Selection;
use crate::ui::Length;
use crate::view::CursorWindow;
use crate::view::RenderArea;
use crate::view::context::Menu;
use bon::builder;

/// Returns the overview of a track (the horizontally scrollable section of clip overviews).
#[builder]
pub fn overview(
    track: &Track,
    selection: &Selection,
    offset_mapping: OffsetMapping,
    time_context: Changing<TimeContext>,
    negative_overview_offset: Length,
    cursor: Instant,
    player: Option<Player>,
    held_clip: Option<Id<Clip>>,
) -> View {
    let clips = View::Layers(
        track
            .clips
            .values()
            .map(|clip| {
                if held_clip.is_some_and(|id| id == clip.id()) {
                    return View::Empty;
                }

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

                overview.quoted(clip_width).x_positioned(clip_offset)
            })
            .collect(),
    );

    let object_acceptor = {
        let offset_mapping = offset_mapping.clone();
        let track = track.id;

        move |holdable, render_area: RenderArea| {
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
    };

    let background = View::Empty
        .contextual(Menu::track_overview())
        .grabbable(|render_area| {
            Some(Holdable::SelectionBox {
                start: render_area.mouse_position,
            })
        })
        .object_accepting(object_acceptor)
        .selectable(Selectable::Track(track.id));

    View::Layers(vec![
        background,
        clips,
        CursorWindow::builder()
            .cursor(cursor)
            .offset_mapping(offset_mapping)
            .maybe_player(player)
            .time_context(time_context)
            .window_offset(negative_overview_offset)
            .build()
            .view(),
    ])
    .scrollable(Action::MoveOverview)
}
