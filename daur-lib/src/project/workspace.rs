use crate::project::{Action, ADD_TRACK_DESCRIPTION, ADD_TRACK_LABEL};
use crate::time::Instant;
use crate::track::{Overview, Track};
use crate::ui::{Length, Offset};
use crate::widget::heterogeneous::TwoStack;
use crate::widget::homogenous::Stack;
use crate::widget::{Bordered, Button, Hoverable, OnClick, Ruler, Text, ToWidget};
use crate::{time, track, ui};
use arcstr::literal;
use ratatui::layout::Constraint;
use saturating_cast::SaturatingCast as _;
use std::sync::Arc;

/// An overview of all the tracks in a project.
#[derive(Debug)]
pub struct Workspace {
    /// How much the overview if offset to the right.
    pub overview_offset: Offset,
    /// The index of the selected track.
    pub selected_track_index: usize,
    /// The index of the selected clip.
    pub selected_clip_index: usize,
    /// The width of the track settings area.
    pub track_settings_width: Length,
    /// The tracks in the project.
    pub tracks: Vec<Arc<Track>>,
    /// The ui mapping used for the overview.
    pub ui_mapping: ui::Mapping,
    /// The time mapping used for the overview.
    pub time_mapping: time::Mapping,
    /// The position of the musical cursor.
    pub cursor: Instant,
}

impl ToWidget for Workspace {
    type Widget<'widget> = TwoStack<
        TwoStack<Text, Ruler>,
        TwoStack<
            TwoStack<Stack<track::Settings>, Button<'static, Hoverable<Bordered<Text>>>>,
            Stack<Overview>,
        >,
    >;

    fn to_widget(&self) -> Self::Widget<'_> {
        let track_count = self.tracks.len().saturating_cast();

        let horizontal_constraints = [self.track_settings_width.constraint(), Constraint::Fill(1)];
        let ruler_constraints = [Constraint::Length(2), Constraint::Fill(1)];

        // TODO: put something here?
        let empty_space = Text::centred(literal!(":)"));

        let ruler = Ruler {
            mapping: self.ui_mapping.clone(),
            offset: self.overview_offset,
        };
        let ruler_row = TwoStack::horizontal((empty_space, ruler), horizontal_constraints);

        let mut track_settings = Vec::new();
        let mut track_overviews = Vec::new();

        for (index, track) in self.tracks.iter().map(Arc::clone).enumerate() {
            let selected = index == self.selected_track_index;
            track_settings.push(track.settings(selected, index));
            track_overviews.push(Overview {
                track,
                selected_clip_index: self.selected_clip_index,
                time_mapping: self.time_mapping.clone(),
                ui_mapping: self.ui_mapping.clone(),
                offset: self.overview_offset,
                cursor: self.cursor,
                index,
            });
        }

        // A "dummy-track" for the row with the add track button
        track_overviews.push(Overview {
            track: Arc::new(Track::new()),
            selected_clip_index: self.selected_clip_index,
            time_mapping: self.time_mapping.clone(),
            ui_mapping: self.ui_mapping.clone(),
            offset: self.overview_offset,
            cursor: self.cursor,
            index: usize::MAX,
        });

        let add_track_button = Button::described(
            ADD_TRACK_LABEL,
            ADD_TRACK_DESCRIPTION,
            OnClick::from(Action::AddTrack),
        );

        let settings_column = TwoStack::vertical(
            (Stack::equisized_vertical(track_settings), add_track_button),
            [Constraint::Fill(track_count), Constraint::Fill(1)],
        );
        let overview_column = Stack::equisized_vertical(track_overviews);

        let track_area =
            TwoStack::horizontal((settings_column, overview_column), horizontal_constraints);

        TwoStack::vertical((ruler_row, track_area), ruler_constraints)
    }
}
