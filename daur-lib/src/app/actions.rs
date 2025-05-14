use crate::Action;

/// A queue of actions to be taken.
#[must_use = "actions must be processed"]
#[derive(Clone, Debug, Default)]
pub struct Actions {
    actions: Vec<Action>,
}

impl Actions {
    /// Constructs an empty action queue.
    pub const fn new() -> Actions {
        Actions {
            actions: Vec::new(),
        }
    }

    /// Adds an action to the queue.
    pub fn push(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub(crate) fn into_vec(self) -> Vec<Action> {
        self.actions
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}
