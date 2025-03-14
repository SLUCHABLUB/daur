use crate::app::Action;
use crate::popup::Popup;
use crate::ui::Size;
use crate::widget::{Button, HasSize, OnClick, Ref, ToWidget, Widget};
use educe::Educe;
use std::sync::Weak;

/// A button that also closes the containing popup
#[derive(Clone, Debug, Educe)]
#[educe(Eq, PartialEq)]
pub struct Terminating<Content> {
    pub content: Content,
    /// The id of the containing popup
    #[educe(Eq(ignore))]
    pub popup: Weak<Popup>,
}

impl<Child> Terminating<Child> {
    pub fn popup(&self) -> Weak<Popup> {
        Weak::clone(&self.popup)
    }
}

impl<Content: Widget> ToWidget for Terminating<Content> {
    type Widget<'widget>
        = Button<'static, Ref<'widget, Content>>
    where
        Content: 'widget;

    fn to_widget(&self) -> Self::Widget<'_> {
        Button {
            on_click: OnClick::from(Action::ClosePopup(self.popup())),
            content: Ref::from(&self.content),
        }
    }
}

impl<Child: HasSize> HasSize for Terminating<Child> {
    fn size(&self) -> Size {
        self.content.size()
    }
}
