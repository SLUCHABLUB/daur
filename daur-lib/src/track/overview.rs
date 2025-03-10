use crate::app::Action;
use crate::popup::Popup;
use crate::time::{Instant, Period};
use crate::track::Track;
use crate::ui::{Length, Offset, Point, Rectangle};
use crate::widget::heterogeneous::Layers;
use crate::widget::{CursorWindow, Direction, Feed, SizeInformed, ToWidget, Widget};
use crate::{clip, project, time, ui};
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

impl ToWidget for Overview {
    type Widget<'widget> =
        SizeInformed<'widget, Layers<(Feed<'widget, Option<clip::Overview>>, CursorWindow)>>;

    fn to_widget(&self) -> Self::Widget<'_> {
        SizeInformed::new(move |size| {
            let overview_period = self
                .ui_mapping
                .period((-self.offset).saturate(), size.width);

            Layers::new((
                Feed::new(Direction::Right, self.offset, move |index| {
                    let Ok(index) = usize::try_from(index) else {
                        return (None, self.offset.abs());
                    };

                    let (clip_index, parity) = index.div_rem(&2);

                    // if index is even
                    if parity == 0 {
                        // Return the space between clips

                        let last_clip_end = clip_index
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
                            .nth(clip_index)
                            .map_or(Length::MAX, |instant| self.ui_mapping.offset(*instant));

                        let size = next_clip_start - last_clip_end;

                        return (None, size);
                    }

                    let Some((start, clip)) = self.track.clips.iter().nth(clip_index) else {
                        return (None, Length::MAX);
                    };

                    let clip_period = clip.period(*start, &self.time_mapping);
                    let clip_width = self.ui_mapping.width_of(clip_period);

                    let Some(visible_period) = Period::intersection(overview_period, clip_period)
                    else {
                        return (None, clip_width);
                    };

                    let selected = self.selected_clip_index == clip_index;

                    let overview = clip::Overview {
                        clip: Arc::clone(clip),
                        selected,
                        track_index: self.index,
                        index,
                        period: clip_period,
                        visible_period,
                    };

                    (Some(overview), clip_width)
                }),
                CursorWindow {
                    mapping: self.ui_mapping.clone(),
                    offset: self.offset,
                    instant: self.cursor,
                },
            ))
        })
    }
}

impl Widget for Overview {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        self.to_widget().render(area, buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        // TODO: move, select or open clips
        let _ = area;

        if button == MouseButton::Left {
            actions.push(Action::SelectTrack(self.index));
        }

        if button == MouseButton::Right {
            actions.push(Action::OpenPopup(right_click_menu().at(position)));
        }

        self.to_widget().click(area, button, position, actions);
    }
}
