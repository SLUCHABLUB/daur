use crate::popup::button::TerminatingButton;
use crate::popup::info::PopupInfo;
use crate::widget::homogenous_stack::HomogenousStack;
use crate::widget::sized::Sized;
use crate::widget::Widget;
use ratatui::layout::Size;

#[derive(Clone, Eq, PartialEq)]
pub struct ButtonPanel {
    pub info: PopupInfo,
    pub buttons: Vec<TerminatingButton>,
}

impl ButtonPanel {
    pub fn size(&self) -> Size {
        let mut width = 0;
        let mut height = 0;

        for button in &self.buttons {
            let size = button.button.size();

            width = u16::max(width, size.width);
            height += size.height;
        }

        Size { width, height }
    }

    pub fn to_widget(&self) -> impl Widget + use<'_> {
        HomogenousStack::equidistant_vertical(&self.buttons)
    }
}
