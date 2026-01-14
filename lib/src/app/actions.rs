//! Items pertaining to [`Actions`].

use crate::app::Action;

/// A queue of actions to be taken.
#[must_use = "actions must be processed"]
#[derive(Clone, Debug, Default)]
pub struct Actions {
    /// The underlying queue.
    queue: Vec<Action>,
}

impl Actions {
    /// Constructs an empty action queue.
    pub const fn new() -> Actions {
        Actions { queue: Vec::new() }
    }

    /// Adds an action to the queue.
    pub fn push(&mut self, action: Action) {
        self.queue.push(action);
    }

    /// Returns an iterator over the actions.
    pub(crate) fn into_iter(self) -> impl Iterator<Item = Action> {
        self.queue.into_iter()
    }

    /// Return whether the action queue is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
