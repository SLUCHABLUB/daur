use crate::popup::button::TerminatingButton;
use crate::popup::info::PopupInfo;
use crate::widget::homogenous_stack::HomogenousStack;
use crate::widget::Widget;
use ratatui::layout::Size;
use saturating_cast::SaturatingCast;

#[derive(Clone, Debug)]
pub struct ButtonPanel {
    pub info: PopupInfo,
    pub buttons: Vec<TerminatingButton>,
    pub unimportant: bool,
}

impl ButtonPanel {
    pub fn size(&self) -> Size {
        let mut width = 0;
        let mut height = 0;

        for button in &self.buttons {
            let mut button_width = 0;

            button_width += usize::max(
                button.button.label.chars().count(),
                button.button.description.chars().count(),
            )
            .saturating_cast::<u16>();

            height += 1;

            if button.button.bordered {
                button_width += 2;
                height += 2;
            }

            width = u16::max(width, button_width);
        }

        Size { width, height }
    }

    pub fn to_widget(&self) -> impl Widget + use<'_> {
        HomogenousStack::equidistant_vertical(&self.buttons)
    }
}
