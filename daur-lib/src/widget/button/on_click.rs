use crate::receiver::Receiver;
use crate::ui::{Point, Size};
use crate::{project, Action};
use crossterm::event::MouseButton;
use std::any::type_name;
use std::fmt;
use std::fmt::{Debug, Formatter};

// TODO: make thin into a trait alias when they get stabilized
pub trait OnClickFunction<'closure>:
    Fn(MouseButton, Size, Point, &mut dyn Receiver<Action>) + Send + Sync + 'closure
{
}

impl<'closure, F> OnClickFunction<'closure> for F where
    F: Fn(MouseButton, Size, Point, &mut dyn Receiver<Action>) + Send + Sync + 'closure
{
}

/// An action to take when a button is clicked
#[derive(Default)]
pub struct OnClick<'closure> {
    function: Option<Box<dyn OnClickFunction<'closure>>>,
}

impl<'closure> OnClick<'closure> {
    /// Construct a new [`OnClick`] from a function.
    pub fn new<F: OnClickFunction<'closure>>(function: F) -> Self {
        OnClick {
            function: Some(Box::new(function)),
        }
    }

    /// Runs the function.
    pub fn run(
        &self,
        mouse_button: MouseButton,
        size: Size,
        position: Point,
        receiver: &mut dyn Receiver<Action>,
    ) {
        if let Some(function) = self.function.as_ref() {
            function(mouse_button, size, position, receiver);
        }
    }
}

impl From<Action> for OnClick<'_> {
    fn from(action: Action) -> Self {
        OnClick::new(move |_, _, _, actions| actions.send(action.clone()))
    }
}

impl From<project::Action> for OnClick<'_> {
    fn from(action: project::Action) -> Self {
        OnClick::new(move |_, _, _, actions| actions.send(Action::Project(action.clone())))
    }
}

impl Debug for OnClick<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<OnClick>())
            .field("function", &"(function)")
            .finish()
    }
}
