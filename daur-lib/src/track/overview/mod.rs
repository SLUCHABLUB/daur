mod cursor;

use crate::app::Action;
use crate::popup::Popup;
use crate::time::Instant;
use crate::track::overview::cursor::Cursor;
use crate::track::Track;
use crate::ui::{Length, NonZeroLength, Offset, Point, Rectangle, Size};
use crate::widget::{Direction, Feed, Widget};
use crate::{project, time, ui};
use arcstr::{literal, ArcStr};
use crossterm::event::MouseButton;
use num::Integer as _;
use ratatui::buffer::Buffer;
use ratatui_explorer::File;
use std::sync::Arc;

const IMPORT_AUDIO: ArcStr = literal!("import audio");
const ADD_NOTES: ArcStr = literal!("add notes");
const OPEN_PIANO_ROLL: ArcStr = literal!("open piano roll");

pub fn open_import_audio_popup() -> Action {
    let action = move |file: &File| Action::import_audio(file.path());

    Action::OpenPopup(Popup::explorer(IMPORT_AUDIO, action))
}

fn right_click_menu() -> Arc<Popup> {
    Popup::unimportant_buttons([
        (IMPORT_AUDIO, open_import_audio_popup()),
        (ADD_NOTES, Action::Project(project::Action::AddNotes)),
        (OPEN_PIANO_ROLL, Action::OpenPianoRoll),
    ])
}

pub struct Overview {
    pub track: Arc<Track>,
    pub selected_clip_index: usize,
    pub time_mapping: time::Mapping,
    pub ui_mapping: ui::Mapping,
    pub offset: Offset,
    pub cursor: Instant,
    pub index: usize,
}

impl Widget for Overview {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let window_end = Offset::from(area.position.x + area.size.width);

        // TODO: alternate background colour for grid

        // Render the clips
        Feed::new(Direction::Right, self.offset, |index| {
            let Ok(index) = usize::try_from(index) else {
                return (None, self.offset.abs());
            };

            let (index, parity) = index.div_rem(&2);

            // if index is even
            if parity == 0 {
                // Return the space between clips

                let last_clip_end = index
                    .checked_sub(1)
                    .and_then(|index| {
                        let (start, clip) = self.track.clips.iter().nth(index)?;
                        let end = clip.period(*start, &self.time_mapping).end();
                        Some(self.ui_mapping.offset(end))
                    })
                    .unwrap_or(Length::ZERO);

                let next_clip_start = self
                    .track
                    .clips
                    .keys()
                    .nth(index)
                    .map_or(Length::MAX, |instant| self.ui_mapping.offset(*instant));

                let size = next_clip_start - last_clip_end;

                return (None, size);
            }

            let Some((start, clip)) = self.track.clips.iter().nth(index) else {
                return (None, Length::MAX);
            };

            let period = clip.period(*start, &self.time_mapping);

            let clip_start = Offset::from(self.ui_mapping.offset(period.start)) + self.offset;
            let clip_end = Offset::from(self.ui_mapping.offset(period.end())) + self.offset;
            let clip_width = clip_end - clip_start;

            let Some(clip_width) = NonZeroLength::from_length(clip_width.saturate()) else {
                return (None, Length::ZERO);
            };

            let [mut x, y] = clip.content.full_overview_viewport();
            let full_width = x[1] - x[0];

            if clip_start < Offset::ZERO {
                // The fraction of the clip that is outside the window (on the left)
                let fraction = (clip_start.abs() / clip_width).to_float();
                x[0] += fraction * full_width;
            }
            if window_end < clip_end {
                let delta = (clip_end - window_end).saturate();
                // The fraction of the clip that is outside the window (on the right)
                let fraction = (delta / clip_width).to_float();
                x[1] -= fraction * full_width;
            }

            let selected = self.selected_clip_index == index;

            (
                Some(clip.overview_canvas(selected).x_bounds(x).y_bounds(y)),
                clip_width.get(),
            )
        })
        .render(area, buffer, mouse_position);

        // TODO: use a Z-stack
        // Render the cursor
        let cursor_offset = self.ui_mapping.offset(self.cursor);
        let cursor_offset = Offset::from(cursor_offset) + self.offset;
        let Some(cursor_offset) = cursor_offset.to_length() else {
            return;
        };
        if area.size.width <= cursor_offset {
            return;
        }

        let cursor_area = Rectangle {
            position: Point {
                x: cursor_offset + area.position.x,
                y: area.position.y,
            },
            size: Size {
                width: Length::CURSOR_WIDTH,
                height: area.size.height,
            },
        };

        Cursor.render(cursor_area, buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        // TODO: move, select or open clips

        let ui_offset = Offset::from(position.x - area.position.x) - self.offset;
        let instant = self.ui_mapping.instant_on_grid(ui_offset.saturate());

        if button == MouseButton::Left {
            actions.push(Action::MoveCursor(instant));
            actions.push(Action::SelectTrack(self.index));
        }

        if button == MouseButton::Right {
            actions.push(Action::OpenPopup(right_click_menu().at(position)));
        }
    }
}
