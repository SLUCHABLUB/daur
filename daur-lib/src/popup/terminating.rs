use crate::app::Action;
use crate::popup::Popup;
use crate::ui::Size;
use crate::view::{Button, Composition, HasSize, OnClick, Ref, View};
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

impl<Content: View> Composition for Terminating<Content> {
    type Body<'view>
        = Button<'static, Ref<'view, Content>>
    where
        Content: 'view;

    fn body(&self) -> Self::Body<'_> {
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
