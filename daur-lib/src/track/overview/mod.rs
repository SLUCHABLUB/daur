mod cursor;

use crate::app::Action;
use crate::popup::Popup;
use crate::project;
use crate::project::changing::Changing;
use crate::time::{Instant, Signature, Tempo};
use crate::track::overview::cursor::Cursor;
use crate::track::Track;
use crate::ui::{Grid, Length, Mapping, NonZeroLength, Offset, Point, Rectangle};
use crate::widget::Widget;
use arcstr::{literal, ArcStr};
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui_explorer::File;
use std::sync::Arc;

const IMPORT_AUDIO: ArcStr = literal!("import audio");
const ADD_NOTES: ArcStr = literal!("add notes");

pub fn open_import_audio_popup() -> Action {
    let action = move |file: &File| Action::import_audio(file.path());

    Action::OpenPopup(Popup::explorer(IMPORT_AUDIO, action))
}

fn right_click_menu() -> Arc<Popup> {
    Popup::unimportant_buttons([
        (IMPORT_AUDIO, open_import_audio_popup()),
        (ADD_NOTES, Action::Project(project::Action::AddNotes)),
    ])
}

pub struct Overview {
    pub track: Arc<Track>,
    pub selected_clip_index: usize,
    pub time_signature: Arc<Changing<Signature>>,
    pub tempo: Arc<Changing<Tempo>>,
    pub grid: Grid,
    pub offset: Length,
    pub cursor: Instant,
    pub index: usize,
}

impl Widget for Overview {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        let window_end = Offset::from(area.x + area.width);

        let time_signature = Arc::clone(&self.time_signature);

        let mapping = Mapping {
            time_signature,
            grid: self.grid,
            offset: self.offset,
        };

        // TODO: alternate background colour for grid

        // Render the clips
        for (index, (start, clip)) in self.track.clips.iter().enumerate() {
            let period = clip.period(*start, &self.time_signature, &self.tempo);

            let clip_start = mapping.offset(period.start);
            let clip_end = mapping.offset(period.end());
            let clip_width = clip_start - clip_end;

            let Some(clip_width) = NonZeroLength::from_length(clip_width.saturate()) else {
                continue;
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

            let clip_area = Rectangle {
                x: clip_start.saturate(),
                y: area.y,
                width: clip_width.get(),
                height: area.height,
            };

            let selected = self.selected_clip_index == index;

            clip.overview_canvas(selected)
                .x_bounds(x)
                .y_bounds(y)
                .render(
                    Rectangle::intersection(clip_area, area),
                    buf,
                    mouse_position,
                );
        }

        // Render the cursor
        if let Some(cursor_column) = mapping.offset_in_range(self.cursor, area.width) {
            let area = Rectangle {
                x: cursor_column,
                y: area.y,
                width: Length::CURSOR_WIDTH,
                height: area.height,
            };

            Cursor.render(area, buf, mouse_position);
        }
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        // TODO: move, select or open clips

        let time_signature = Arc::clone(&self.time_signature);

        let mapping = Mapping {
            time_signature,
            grid: self.grid,
            offset: self.offset,
        };

        let instant = mapping.instant_on_grid(position.x - area.x);

        if button == MouseButton::Left {
            actions.push(Action::MoveCursor(instant));
            actions.push(Action::SelectTrack(self.index));
        }

        // TODO: if clip is selected, open its right-click-menu
        if button == MouseButton::Right {
            actions.push(Action::OpenPopup(right_click_menu().at(position)));
        }
    }
}
