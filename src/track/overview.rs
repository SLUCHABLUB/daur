use crate::app::action::Action;
use crate::app::settings::OverviewSettings;
use crate::app::window::Window;
use crate::clip::Clip;
use crate::id::Id;
use crate::popup::Popup;
use crate::project::changing::Changing;
use crate::time::instant::Instant;
use crate::time::signature::TimeSignature;
use crate::time::tempo::Tempo;
use crate::track::Track;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::symbols::line::VERTICAL;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui_explorer::File;

const IMPORT_AUDIO: &str = "import audio";

pub fn right_click_menu(track: Id<Track>) -> Popup {
    // TODO
    let action = move |file: &File| Action::ImportAudio {
        file: file.path().clone(),
        track,
    };

    Popup::unimportant_buttons([(
        IMPORT_AUDIO,
        Action::OpenPopup(Box::new(Popup::explorer(IMPORT_AUDIO.to_owned(), action))),
    )])
}

pub struct Overview<'a> {
    pub track: &'a Track,
    pub selected_clip: Id<Clip>,
    pub time_signature: &'a Changing<TimeSignature>,
    pub tempo: &'a Changing<Tempo>,
    pub settings: OverviewSettings,
    pub cursor: Instant,
}

impl Overview<'_> {
    fn window(&self, area: Rect) -> Window {
        Window {
            time_signature: self.time_signature,
            overview_settings: self.settings,
            x: area.x,
            width: area.width,
        }
    }
}

impl Widget for Overview<'_> {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let area_end = i32::from(area.x + area.width);

        let window = self.window(area);

        // TODO: alternate background colour for grid

        // Render the clips
        for (start, clip) in &self.track.clips {
            let clip_area = window.period_to_unchecked_rect(
                clip.period(*start, self.time_signature, self.tempo),
                area.x,
                area.y,
                area.height,
            );
            let clip_area_end = clip_area.x + i32::from(clip_area.width);

            let [mut x, y] = clip.content.full_overview_viewport();
            let full_width = x[1] - x[0];

            if clip_area.x < 0 {
                // The fraction of the clip that is outside the window (on the left)
                let fraction = f64::from(clip_area.x).abs() / f64::from(clip_area.width);
                x[0] += fraction * full_width;
            }
            if clip_area_end > area_end {
                let delta = clip_area_end - area_end;
                // The fraction of the clip that is outside the window (on the right)
                let fraction = f64::from(delta) / f64::from(clip_area.width);
                x[1] -= fraction * full_width;
            }

            let selected = clip.id == self.selected_clip;

            clip.overview_canvas(selected)
                .x_bounds(x)
                .y_bounds(y)
                .render(
                    Rect::intersection(clip_area.clamp(), area),
                    buf,
                    mouse_position,
                );
        }

        // Render the cursor
        if let Some(cursor_column) = window.instant_to_column(self.cursor) {
            let area = Rect {
                x: cursor_column,
                y: area.y,
                width: 1,
                height: area.height,
            };
            Paragraph::new(vec![Line::raw(VERTICAL); area.height as usize]).render(
                area,
                buf,
                mouse_position,
            );
        }
    }

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
        // TODO: move, select or open clips

        let window = self.window(area);

        let instant = window.column_to_instant_on_grid(position.x);

        if button == MouseButton::Left {
            action_queue.push(Action::MoveCursor(instant));
        }

        // TODO: && clip not clicked
        if button == MouseButton::Right {
            action_queue.push(Action::OpenPopup(Box::new(
                right_click_menu(self.track.id).at(position),
            )));
        }
    }
}
