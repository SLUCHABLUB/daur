pub(crate) trait OptionExt<T> {
    fn get_or_try_insert_with<F, E>(&mut self, f: F) -> Result<&mut T, E>
    where
        F: FnOnce() -> Result<T, E>;
}

impl<T> OptionExt<T> for Option<T> {
    #[expect(clippy::unwrap_in_result, reason = "`E` may not be populated")]
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
