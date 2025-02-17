use crate::popup::button::Terminating;
use crate::popup::info::PopupInfo;
use crate::widget::button::Button;
use crate::widget::homogenous_stack::HomogenousStack;
use crate::widget::to_widget::ToWidget;

#[derive(Clone, Eq, PartialEq)]
pub struct ButtonPanel {
    pub info: PopupInfo,
    pub buttons: Vec<Terminating<Button>>,
}

impl ToWidget for ButtonPanel {
    type Widget<'buttons> = HomogenousStack<&'buttons Terminating<Button>>;

    fn to_widget(&self) -> Self::Widget<'_> {
        HomogenousStack::equidistant_vertical(&self.buttons)
    }
}
