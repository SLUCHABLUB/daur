mod on_click;

pub use on_click::OnClick;

use crate::view::Alignment;
use crate::view::ToText as _;
use crate::view::View;
use arcstr::ArcStr;

impl View {
    /// Constructs a simple button with no border and left aligned text.
    pub(crate) fn simple_button(label: ArcStr, on_click: OnClick) -> View {
        label.aligned_to(Alignment::TopLeft).on_click(on_click)
    }

    /// Constructs a standard button with a border and centered text.
    pub(crate) fn standard_button(label: ArcStr, on_click: OnClick) -> View {
        label.centred().bordered().on_click(on_click)
    }

    /// Constructs a button with a (maybe thick) border and centered text.
    pub(crate) fn toggle(label: ArcStr, on_click: OnClick, state: bool) -> View {
        label
            .centred()
            .bordered_with_thickness(state)
            .on_click(on_click)
    }

    /// Constructs a button with a description, border and centred text.
    pub(crate) fn described_button(label: ArcStr, description: ArcStr, on_click: OnClick) -> View {
        View::hoverable(label.centred().bordered(), description.centred().bordered())
            .on_click(on_click)
    }
}
