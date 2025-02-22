use crate::widget::Widget;

pub trait ToWidget {
    type Widget<'widget>: Widget
    where
        Self: 'widget;

    fn to_widget(&self) -> Self::Widget<'_>;
}
