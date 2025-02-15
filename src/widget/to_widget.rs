use crate::widget::Widget;

pub trait ToWidget {
    type Widget<'a>: Widget
    where
        Self: 'a;

    fn to_widget(&self) -> Self::Widget<'_>;
}
