mod on_click;

pub use on_click::OnClick;

use crate::view::{Alignment, ToText as _, View};
use arcstr::ArcStr;

impl View {
    /// Turns the view into a button.
    pub fn on_click(self, on_click: OnClick) -> Self {
        View::Button {
            on_click,
            content: Box::new(self),
        }
    }

    /// Constructs a simple button with no border and left aligned text
    pub fn simple_button(label: ArcStr, on_click: OnClick) -> Self {
        label.aligned_to(Alignment::TopLeft).on_click(on_click)
    }

    /// Constructs a standard button with a border and centered text
    pub fn standard_button(label: ArcStr, on_click: OnClick) -> Self {
        label.centred().bordered().on_click(on_click)
    }

    /// Constructs a button with a description, border and centred text
    pub fn described_button(label: ArcStr, description: ArcStr, on_click: OnClick) -> Self {
        View::hoverable(label.centred().bordered(), description.centred().bordered())
            .on_click(on_click)
    }
}
