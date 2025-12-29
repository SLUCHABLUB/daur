use crate::app::Action;
use crate::app::Actions;
use crate::project;
use crate::view::RenderArea;
use derive_more::Debug;

type OnClickFunction = dyn Fn(RenderArea, &mut Actions) + Send + Sync;

/// A function to run when a button is (left) clicked.
#[derive(Default, Debug)]
pub struct OnClick {
    #[debug(skip)]
    function: Option<Box<OnClickFunction>>,
}

impl OnClick {
    /// Construct a new function.
    pub fn new<F: Fn(RenderArea, &mut Actions) + Send + Sync + 'static>(function: F) -> OnClick {
        OnClick {
            function: Some(Box::new(function)),
        }
    }

    /// Creates a new function from a closure generating an [action](Action).
    ///
    /// [`OnClick`] also implements [`From<Action>`] so if the action is available at call-time,
    /// [`from`](From::<Action>::from) is preferred.
    pub fn action<F: Fn() -> Action + Send + Sync + 'static>(generator: F) -> OnClick {
        OnClick::new(move |_, actions| actions.push(generator()))
    }

    /// Runs the function.
    pub fn run(&self, render_area: RenderArea, receiver: &mut Actions) {
        if let Some(function) = self.function.as_ref() {
            function(render_area, receiver);
        }
    }
}

impl From<Action> for OnClick {
    fn from(action: Action) -> OnClick {
        OnClick::action(move || action.clone())
    }
}

impl From<project::Edit> for OnClick {
    fn from(action: project::Edit) -> OnClick {
        OnClick::from(Action::Edit(action))
    }
}
