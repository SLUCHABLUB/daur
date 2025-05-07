use parking_lot::{MappedRwLockWriteGuard, RwLockWriteGuard};

pub(crate) trait GuardExt<'lock, T>: Sized {
    type Mapped<U: 'lock>: 'lock;

    fn try_map<F, R>(guard: Self, f: F) -> Result<Self::Mapped<R>, Self>
    where
        F: FnOnce(&mut T) -> Option<&mut R>;

    fn map_result<F, O, E>(guard: Self, f: F) -> Result<Self::Mapped<O>, E>
    where
        F: FnOnce(&mut T) -> Result<&mut O, E>,
    {
        let mut error = None;

        #[expect(
            clippy::unwrap_used,
            reason = "if the error is `None`, `try_map` will have returned `Ok`"
        )]
        Self::try_map(guard, |inner| match f(inner) {
            Ok(value) => Some(value),
            Err(err) => {
                error = Some(err);
                None
            }
        })
        .map_err(|_| error.unwrap())
    }
}

impl<'lock, T> GuardExt<'lock, T> for RwLockWriteGuard<'lock, T> {
    type Mapped<U: 'lock> = MappedRwLockWriteGuard<'lock, U>;

    fn try_map<F, R>(guard: Self, f: F) -> Result<Self::Mapped<R>, Self>
    where
        F: FnOnce(&mut T) -> Option<&mut R>,
    {
        RwLockWriteGuard::try_map(guard, f)
    }
}

impl<'lock, T> GuardExt<'lock, T> for MappedRwLockWriteGuard<'lock, T> {
    type Mapped<U: 'lock> = MappedRwLockWriteGuard<'lock, U>;

    fn try_map<F, R>(guard: Self, f: F) -> Result<Self::Mapped<R>, Self>
    where
        F: FnOnce(&mut T) -> Option<&mut R>,
    {
        MappedRwLockWriteGuard::try_map(guard, f)
    }
}
