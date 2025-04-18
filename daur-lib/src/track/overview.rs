use crate::app::Action;
use crate::context::Menu;
use crate::time::{Instant, Period};
use crate::track::Track;
use crate::ui::{Length, Offset};
use crate::view::{Direction, OnClick, Quotated, View, cursor_window, feed};
use crate::{clip, time, ui};
use num::Integer as _;
use std::sync::Arc;

/// Returns the track overview.
pub fn overview(
    track: Arc<Track>,
    index: usize,
    selected_clip_index: usize,
    time_mapping: time::Mapping,
    ui_mapping: ui::Mapping,
    offset: Offset,
    cursor: Instant,
) -> View {
    View::size_informed(move |size| {
        let overview_start = (-offset).rectify();
        let overview_period = ui_mapping.period(overview_start, size.width);

        let time_mapping = time_mapping.clone();
        let cursor_mapping = ui_mapping.clone();
        let ui_mapping = ui_mapping.clone();
        View::Layers(vec![
            feed(
                Direction::Right,
                offset,
                feed_generator(
                    Arc::clone(&track),
                    selected_clip_index,
                    time_mapping,
                    ui_mapping,
                    offset,
                    overview_period,
                ),
            ),
            cursor_window(cursor, cursor_mapping, offset),
        ])
    })
    .on_click(OnClick::new(move |_, _, actions| {
        // TODO: move, select or open clips

        actions.send(Action::SelectTrack(index));
    }))
    .context(Menu::track_overview())
}

/// Returns a function for generating clip-overviews from a feed index.
fn feed_generator(
    track: Arc<Track>,
    selected_clip_index: usize,
    time_mapping: time::Mapping,
    ui_mapping: ui::Mapping,
    offset: Offset,
    overview_period: Period,
) -> impl Fn(isize) -> Quotated {
    move |index| {
        let Ok(index) = usize::try_from(index) else {
            return View::Empty.quotated(offset.abs());
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

            return View::Empty.quotated(size);
        }

        let Some((start, clip)) = track.clips.iter().nth(clip_index) else {
            return View::Empty.fill_remaining();
        };

        let clip_period = clip.period(*start, &time_mapping);
        let clip_width = ui_mapping.width_of(clip_period);

        let Some(visible_period) = Period::intersection(overview_period, clip_period) else {
            return View::Empty.quotated(clip_width);
        };

        let selected = selected_clip_index == clip_index;

        clip::overview(
            Arc::clone(clip),
            index,
            index,
            selected,
            clip_period,
            visible_period,
            time_mapping.clone(),
        )
        .quotated(clip_width)
    }
}
