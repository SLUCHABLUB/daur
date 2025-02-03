use crate::time::duration::Duration;

/// A (temporary or permanent) change in some setting, such as key or time signature.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Change<T> {
    pub value: T,
    /// The duration of the change, or `None` if it's indefinite
    pub duration: Option<Duration>,
}
