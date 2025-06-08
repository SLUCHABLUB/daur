use crate::app::Action;
use crate::audio::Player;
use crate::metre::{Changing, Instant, OffsetMapping, TimeContext};
use crate::project::track::clip::Path;
use crate::project::track::{Clip, clip};
use crate::project::{Edit, Track};
use crate::select::Selection;
use crate::ui::Length;
use crate::view::context::Menu;
use crate::view::{CursorWindow, RenderArea, View};
use crate::{Holdable, Id, Selectable};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub(in crate::project) struct Overview<'track, 'selection> {
    track: &'track Track,
    selection: &'selection Selection,
    offset_mapping: OffsetMapping,
    time_context: Changing<TimeContext>,
    negative_overview_offset: Length,
    cursor: Instant,
    player: Option<Player>,
    held_clip: Option<Id<Clip>>,
}

impl Overview<'_, '_> {
    pub fn view(self) -> View {
        let clips = View::Layers(
            self.track
                .clips
                .values()
                .map(|clip| {
                    if self.held_clip.is_some_and(|id| id == clip.id()) {
                        return View::Empty;
                    }

                    let clip_start = self
                        .track
                        .clip_starts
                        .get(&clip.id())
                        .copied()
                        .unwrap_or_default();
                    let absolute_clip_offset = self.offset_mapping.offset(clip_start);

                    let start_crop = self.negative_overview_offset - absolute_clip_offset;

                    let clip_offset = absolute_clip_offset - self.negative_overview_offset;

                    let clip_end = clip_start + clip.duration().get();
                    let clip_end_offset =
                        self.offset_mapping.offset(clip_end) - self.negative_overview_offset;

                    let selected = self
                        .selection
                        .contains_clip(Path::new(self.track.id, clip.id()));

                    let clip_width = clip_end_offset - clip_offset;

                    let overview = clip::overview(
                        clip,
                        selected,
                        self.offset_mapping.clone(),
                        start_crop,
                        self.track.id,
                    );

                    overview.quotated(clip_width).x_positioned(clip_offset)
                })
                .collect(),
        );

        // TODO: add a selection box
        let background = View::Empty
            .contextual(Menu::track_overview())
            .selectable(Selectable::Track(self.track.id))
            .object_accepting(self.object_acceptor());

        View::Layers(vec![
            background,
            clips,
            CursorWindow::builder()
                .cursor(self.cursor)
                .offset_mapping(self.offset_mapping)
                .player(self.player)
                .time_context(self.time_context)
                .window_offset(self.negative_overview_offset)
                .build()
                .view(),
        ])
        .scrollable(Action::MoveOverview)
    }

    fn object_acceptor(&self) -> impl Fn(Holdable, RenderArea) -> Option<Action> + 'static {
        let offset_mapping = self.offset_mapping.clone();
        let negative_overview_offset = self.negative_overview_offset;
        let track = self.track.id;

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
}
