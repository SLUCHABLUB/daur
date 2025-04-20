use crate::ui::{Point, Size};
use crate::{Action, Receiver, project};
use std::any::type_name;
use std::fmt;
use std::fmt::{Debug, Formatter};

type OnClickFunction = dyn Fn(Size, Point, &mut dyn Receiver<Action>) + Send + Sync + 'static;

/// A function to run when a button is (left) clicked.
#[derive(Default)]
pub struct OnClick {
    function: Option<Box<OnClickFunction>>,
}

impl OnClick {
    /// Construct a new function.
    pub fn new<F: Fn(Size, Point, &mut dyn Receiver<Action>) + Send + Sync + 'static>(
        function: F,
    ) -> Self {
        OnClick {
            function: Some(Box::new(function)),
        }
    }

    /// Creates a new function from a closure generating an [action](Action).
    ///
    /// [`OnClick`] also implements [`From<Action>`] so if the action is available at call-time,
    /// [`from`](From::<Action>::from) is preferred.
    pub fn action<F: Fn() -> Action + Send + Sync + 'static>(generator: F) -> Self {
        OnClick::new(move |_, _, actions| actions.send(generator()))
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
        OnClick::action(move || action.clone())
    }
}

impl From<project::Action> for OnClick {
    fn from(action: project::Action) -> Self {
        OnClick::from(Action::Project(action))
    }
}

impl Debug for OnClick {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<OnClick>())
            .field("function", &"(function)")
            .finish()
    }
}
