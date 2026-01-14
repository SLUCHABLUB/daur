//! Items pertaining to [`OptionExt`].

/// An extension trait for [`Option`].
pub(crate) trait OptionExt<T> {
    /// Returns a mutable reference to the interval value if it exists.
    /// If not, a fallible function is executed the result of which, if it is `Ok`,
    /// is inserted into the option and returned, otherwise the error is returned.
    fn get_or_try_insert_with<F, E>(&mut self, f: F) -> Result<&mut T, E>
    where
        F: FnOnce() -> Result<T, E>;
}

impl<T> OptionExt<T> for Option<T> {
    fn get_or_try_insert_with<F, E>(&mut self, f: F) -> Result<&mut T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        if self.is_none() {
            *self = Some(f()?);
        }

        #[expect(clippy::unwrap_used, reason = "see `get_or_insert_with`")]
        Ok(self.as_mut().unwrap())
    }
}
