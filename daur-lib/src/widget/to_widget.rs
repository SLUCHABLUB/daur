use crate::widget::Widget;

/// A type that can be converted to a widget
pub trait ToWidget {
    /// The widget type
    type Widget<'widget>: Widget
    where
        Self: 'widget;

    /// Returns the widget
    fn to_widget(&self) -> Self::Widget<'_>;
}
