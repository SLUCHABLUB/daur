use crate::app::action::Action;
use crate::key::Key;
use crate::popup::Popup;
use crate::project::{Project, PAUSE, PAUSE_DESCRIPTION, PLAY, PLAY_DESCRIPTION};
use crate::widget::block::Block;
use crate::widget::button::Button;
use crate::widget::heterogeneous_stack::{ThreeStack, TwoStack};
use crate::widget::Widget;
use ratatui::layout::{Constraint, Flex};
use ratatui::widgets::Paragraph;

pub fn select_key(key: Key) -> Action {
    Action::OpenPopup(Popup::key_selector(key))
}

impl Project {
    // TODO: window controls (opening instrument rack, piano roll, et.c)

    // TODO: record, loop, metronome
    // TODO: cursor fine positioning
    // TODO: grid size
    // TODO: master volume
    pub fn bar(&self, playing: bool) -> impl Widget + use<'_> {
        let block = Block::thick(self.title.clone());

        let playback_button = if playing {
            Button::new(PAUSE, Action::Pause)
                .description(PAUSE_DESCRIPTION)
                .bordered()
        } else {
            Button::new(PLAY, Action::Play)
                .description(PLAY_DESCRIPTION)
                .bordered()
        };

        let fallbacks = ThreeStack::horizontal(
            (
                Button::new(
                    self.key.start.get().to_string(),
                    select_key(self.key.start.get()),
                )
                .bordered(),
                Button::new(self.time_signature.start.get().to_string(), Action::None).bordered(),
                Button::new(self.tempo.start.get().to_string(), Action::None).bordered(),
            ),
            [Constraint::Fill(1); 3],
        );

        let left_side = TwoStack::horizontal(
            (Paragraph::new("TODO"), fallbacks),
            [Constraint::Fill(1); 2],
        )
        .flex(Flex::SpaceBetween);

        ThreeStack::horizontal(
            (
                left_side,
                playback_button,
                Paragraph::new("TODO").centered(),
            ),
            [
                Constraint::Fill(1),
                Constraint::Length(7),
                Constraint::Fill(1),
            ],
        )
        .block(block)
        .flex(Flex::Center)
    }
}
