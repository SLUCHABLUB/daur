use crate::app::action::Action;
use crate::app::window::Window;
use crate::app::OverviewSettings;
use crate::clip::Clip;
use crate::length::offset::Offset;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::length::Length;
use crate::popup::Popup;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::tempo::Tempo;
use crate::time::TimeSignature;
use crate::track::Track;
use crate::widget::text::Text;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::symbols::line::VERTICAL;
use ratatui_explorer::File;
use saturating_cast::SaturatingCast as _;
use std::sync::{Arc, Weak};

const IMPORT_AUDIO: &str = "import audio";

pub fn open_import_audio_popup() -> Action {
    let action = move |file: &File| Action::ImportAudio {
        file: file.path().clone(),
    };

    Action::OpenPopup(Popup::explorer(IMPORT_AUDIO.to_owned(), action))
}

fn right_click_menu() -> Arc<Popup> {
    Popup::unimportant_buttons([(IMPORT_AUDIO, open_import_audio_popup())])
}

pub struct Overview<'project> {
    pub track: Arc<Track>,
    pub selected_clip: Weak<Clip>,
    pub time_signature: &'project Changing<TimeSignature>,
    pub tempo: &'project Changing<Tempo>,
    pub settings: OverviewSettings,
    pub cursor: Instant,
    pub index: usize,
}

impl Overview<'_> {
    fn window(&self, area: Rectangle) -> Window {
        Window {
            time_signature: self.time_signature,
            overview_settings: self.settings,
            x: area.x,
            width: area.width,
        }
    }
}

impl Widget for Overview<'_> {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        let area_end = Offset::from(area.x + area.width);

        let window = self.window(area);

        // TODO: alternate background colour for grid

        // Render the clips
        self.track.clips.map(|start, clip| {
            let clip_area = window.period_to_unchecked_rect(
                clip.period(*start, self.time_signature, self.tempo),
                area.x,
                area.y,
                area.height,
            );
            let clip_area_end = clip_area.x + clip_area.width;

            let [mut x, y] = clip.content.full_overview_viewport();
            let full_width = x[1] - x[0];

            if clip_area.x < Offset::ZERO {
                // The fraction of the clip that is outside the window (on the left)
                let fraction = (clip_area.x.abs() / clip_area.width).to_float();
                x[0] += fraction * full_width;
            }
            if area_end < clip_area_end {
                let delta = (clip_area_end - area_end).saturate();
                // The fraction of the clip that is outside the window (on the right)
                let fraction = (delta / clip_area.width).to_float();
                x[1] -= fraction * full_width;
            }

            let selected = self
                .selected_clip
                .upgrade()
                .is_some_and(|upgrade| upgrade == *clip);

            clip.overview_canvas(selected)
                .x_bounds(x)
                .y_bounds(y)
                .render(
                    Rectangle::intersection(clip_area.clamp(), area),
                    buf,
                    mouse_position,
                );
        });

        // Render the cursor
        if let Some(cursor_column) = window.instant_to_column(self.cursor) {
            let area = Rectangle {
                x: cursor_column,
                y: area.y,
                width: Length::CURSOR_WIDTH,
                height: area.height,
            };

            let rows = area.height / Length::CHAR_HEIGHT;
            let rows = rows.round().saturating_cast();

            Text::left_aligned(vec![VERTICAL; rows].join("\n")).render(area, buf, mouse_position);
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

        let window = self.window(area);

        let instant = window.column_to_instant_on_grid(position.x);

        if button == MouseButton::Left {
            actions.push(Action::MoveCursor(instant));
            actions.push(Action::SelectTrack(self.index));
        }

        // TODO: && clip not clicked
        if button == MouseButton::Right {
            actions.push(Action::OpenPopup(right_click_menu().at(position)));
        }
    }
}
