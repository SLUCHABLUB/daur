use crate::popup::button::Terminating;
use crate::popup::info::PopupInfo;
use crate::widget::button::Button;
use crate::widget::homogenous::Stack;
use crate::widget::to_widget::ToWidget;

#[derive(Clone, Eq, PartialEq)]
pub struct ButtonPanel {
    pub info: PopupInfo,
    pub buttons: Vec<Terminating<Button>>,
}

impl ToWidget for ButtonPanel {
    type Widget<'buttons> = Stack<&'buttons Terminating<Button>>;

    fn to_widget(&self) -> Self::Widget<'_> {
        Stack::equidistant_vertical(&self.buttons)
    }
}
