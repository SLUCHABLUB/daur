use std::ops::{Deref, DerefMut};
use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Default)]
pub struct Lock<T> {
    inner: RwLock<T>,
}

pub enum ReadGuard<'lock, T> {
    Guard(RwLockReadGuard<'lock, T>),
    Poison(PoisonError<RwLockReadGuard<'lock, T>>),
}

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ReadGuard::Guard(guard) => guard,
            ReadGuard::Poison(poison) => poison.get_ref(),
        }
    }
}

pub enum WriteGuard<'lock, T> {
    Guard(RwLockWriteGuard<'lock, T>),
    Poison(PoisonError<RwLockWriteGuard<'lock, T>>),
}

impl<T> Deref for WriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            WriteGuard::Guard(guard) => guard,
            WriteGuard::Poison(poison) => poison.get_ref(),
        }
    }
}

impl<T> DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            WriteGuard::Guard(guard) => guard,
            WriteGuard::Poison(poison) => poison.get_mut(),
        }
    }
}

impl<T> Lock<T> {
    pub const fn new(value: T) -> Self {
        Lock {
            inner: RwLock::new(value),
        }
    }

    pub fn read(&self) -> ReadGuard<T> {
        match self.inner.read() {
            Ok(guard) => ReadGuard::Guard(guard),
            Err(poison) => ReadGuard::Poison(poison),
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        match self.inner.write() {
            Ok(guard) => WriteGuard::Guard(guard),
            Err(poison) => WriteGuard::Poison(poison),
        }
    }
}

impl<T: Clone> Clone for Lock<T> {
    fn clone(&self) -> Self {
        Lock::new(self.read().clone())
    }
}
