use crate::receiver::Receiver;
use crate::ui::{Point, Size};
use crate::{project, Action};
use std::any::type_name;
use std::fmt;
use std::fmt::{Debug, Formatter};

type OnClickFunction = dyn Fn(Size, Point, &mut dyn Receiver<Action>) + Send + Sync + 'static;

/// An action to take when a button is clicked
#[derive(Default)]
pub struct OnClick {
    function: Option<Box<OnClickFunction>>,
}

impl OnClick {
    /// Construct a new [`OnClick`] from a function.
    pub fn new<F: Fn(Size, Point, &mut dyn Receiver<Action>) + Send + Sync + 'static>(
        function: F,
    ) -> Self {
        OnClick {
            function: Some(Box::new(function)),
        }
    }

    /// Runs the function.
    pub fn run(&self, size: Size, position: Point, receiver: &mut dyn Receiver<Action>) {
        if let Some(function) = self.function.as_ref() {
            function(size, position, receiver);
        }
    }
}

impl From<Action> for OnClick {
    fn from(action: Action) -> Self {
        OnClick::new(move |_, _, actions| actions.send(action.clone()))
    }
}

impl From<project::Action> for OnClick {
    fn from(action: project::Action) -> Self {
        OnClick::new(move |_, _, actions| actions.send(Action::Project(action.clone())))
    }
}

impl Debug for OnClick {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<OnClick>())
            .field("function", &"(function)")
            .finish()
    }
}
