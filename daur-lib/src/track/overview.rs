use crate::app::Action;
use crate::audio::Player;
use crate::time::{Instant, Period};
use crate::track::Track;
use crate::ui::{Direction, Length, Offset};
use crate::view::context::Menu;
use crate::view::{CursorWindow, OnClick, Quotated, View, feed};
use crate::{Clip, clip, time, ui};
use closure::closure;
use num::Integer as _;
use std::sync::{Arc, Weak};

/// Returns the track overview.
pub(crate) fn overview(
    track: Arc<Track>,
    selected_clip: &Weak<Clip>,
    time_mapping: &time::Mapping,
    ui_mapping: &ui::Mapping,
    offset: Length,
    cursor: Instant,
    player: Option<&Player>,
) -> View {
    View::size_informed(closure!([
        clone selected_clip,
        clone time_mapping,
        clone ui_mapping,
        cloned player,
    ] move |size| {
        let vissible_period_start = offset;
        let vissible_period = ui_mapping.period(vissible_period_start, size.width);

        // TODO: don't use a feed here
        let generator = feed_generator(
            &track,
            &selected_clip,
            &time_mapping,
            &ui_mapping,
            Offset::negative(offset),
            vissible_period,
        );

        View::Layers(vec![
            feed(Direction::Right, Offset::negative(offset), generator),
            CursorWindow::view(player.clone(), cursor, time_mapping.clone(), ui_mapping.clone(), offset),
        ])
    }))
    .contextual(Menu::track_overview())
        .scrollable(Action::MoveOverview)
}

/// Returns a function for generating clip-overviews from a feed index.
fn feed_generator(
    track: &Arc<Track>,
    selected_clip: &Weak<Clip>,
    time_mapping: &time::Mapping,
    ui_mapping: &ui::Mapping,
    offset: Offset,
    visible_period: Period,
) -> impl Fn(isize) -> Quotated + 'static {
    let track_reference = Arc::downgrade(track);

    let empty = move || {
        View::Empty.on_click(OnClick::from(Action::SelectClip {
            track: Weak::clone(&track_reference),
            clip: Weak::new(),
        }))
    };

    closure!([clone track, clone selected_clip, clone time_mapping, clone ui_mapping, clone empty] move |index| {
        let Ok(index) = usize::try_from(index) else {
            return empty().quotated(offset.abs());
        };

        let (clip_index, parity) = index.div_rem(&2);

        // TODO: add spacing to feed
        // if index is even
        if parity == 0 {
            // Return the space between clips

            let last_clip_end = clip_index
                .checked_sub(1)
                .and_then(|index| {
                    let (start, clip) = track.clips.iter().nth(index)?;
                    let end = clip.period(*start, &time_mapping).end();
                    Some(ui_mapping.x_offset(end))
                })
                .unwrap_or(Length::ZERO);

            let next_clip_start = track
                .clips
                .keys()
                .nth(clip_index)
                .map_or(last_clip_end, |instant| ui_mapping.x_offset(*instant));

            let size = next_clip_start - last_clip_end;

            return empty().quotated(size);
        }

        let Some((start, clip)) = track.clips.iter().nth(clip_index) else {
            return empty().fill_remaining();
        };
        let clip_reference = Arc::downgrade(clip);

        let clip_period = clip.period(*start, &time_mapping);
        let clip_width = ui_mapping.width_of(clip_period);

        let Some(visible_period) = Period::intersection(visible_period, clip_period) else {
            return empty().quotated(clip_width);
        };

        let selected = selected_clip.as_ptr() == clip_reference.as_ptr();

        clip::overview(
            Arc::clone(clip),
            Arc::downgrade(&track),
            selected,
            clip_period,
            visible_period,
            time_mapping.clone(),
        )
        .quotated(clip_width)
    })
}
