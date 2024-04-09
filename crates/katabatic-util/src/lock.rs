use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Weak,
    },
};

use parking_lot::*;

#[derive(Debug, Default)]
pub struct Lock<T: ?Sized>(RwLock<T>);

impl<T> Lock<T> {
    pub fn new(value: T) -> Self {
        Self(RwLock::new(value))
    }

    pub fn read(&self) -> Read<'_, T> {
        Read::new(self)
    }

    pub fn try_read(&self) -> Option<Read<'_, T>> {
        Read::try_new(self)
    }

    pub fn write(&self) -> Write<'_, T> {
        Write::new(self)
    }

    pub fn try_write(&self) -> Option<Write<'_, T>> {
        Write::try_new(self)
    }

    pub fn read_write(&self) -> ReadWrite<'_, T> {
        ReadWrite::new(self)
    }

    pub fn map_read<U, F>(&self, f: F) -> MapRead<'_, U>
    where
        F: FnOnce(&T) -> &U,
    {
        MapRead::new(self, f)
    }

    pub fn map_write<U, F>(&self, f: F) -> MapWrite<'_, U>
    where
        F: FnOnce(&mut T) -> &mut U,
    {
        MapWrite::new(self, f)
    }

    pub fn try_map_read<U, F>(&self, f: F) -> Option<MapRead<'_, U>>
    where
        F: FnOnce(&T) -> &U,
    {
        MapRead::try_new(self, f)
    }

    pub fn try_map_write<U, F>(&self, f: F) -> Option<MapWrite<'_, U>>
    where
        F: FnOnce(&mut T) -> &mut U,
    {
        MapWrite::try_new(self, f)
    }
}

impl<T: Clone> Clone for Lock<T> {
    fn clone(&self) -> Self {
        Self(RwLock::new(self.0.read().clone()))
    }
}

impl<T: Clone> From<T> for Lock<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[derive(Debug)]
pub struct Read<'a, T>(RwLockReadGuard<'a, T>);
#[derive(Debug)]
pub struct Write<'a, T>(RwLockWriteGuard<'a, T>);
#[derive(Debug)]
pub struct ReadWrite<'a, T>(RwLockUpgradableReadGuard<'a, T>);

impl<'a, T> Read<'a, T> {
    pub fn new(lock: &'a Lock<T>) -> Self {
        Self(lock.0.read())
    }

    pub fn try_new(lock: &'a Lock<T>) -> Option<Self> {
        lock.0.try_read().map(Self)
    }

    pub fn into_inner(self) -> RwLockReadGuard<'a, T> {
        self.0
    }

    pub fn map_read<U, F>(self, f: F) -> MapRead<'a, U>
    where
        F: FnOnce(&T) -> &U,
    {
        MapRead(RwLockReadGuard::map(self.0, f))
    }
}

impl<'a, T> Write<'a, T> {
    pub fn new(lock: &'a Lock<T>) -> Self {
        Self(lock.0.write())
    }

    pub fn try_new(lock: &'a Lock<T>) -> Option<Self> {
        lock.0.try_write().map(Self)
    }

    pub fn into_inner(self) -> RwLockWriteGuard<'a, T> {
        self.0
    }

    pub fn map_write<U, F>(self, f: F) -> MapWrite<'a, U>
    where
        F: FnOnce(&mut T) -> &mut U,
    {
        MapWrite(RwLockWriteGuard::map(self.0, f))
    }
}

impl<'a, T> ReadWrite<'a, T> {
    pub fn new(lock: &'a Lock<T>) -> Self {
        Self(lock.0.upgradable_read())
    }

    pub fn into_inner(self) -> RwLockUpgradableReadGuard<'a, T> {
        self.0
    }

    pub fn upgrade(self) -> Write<'a, T> {
        Write(RwLockUpgradableReadGuard::upgrade(self.0))
    }
}

impl<'a, T> Deref for Read<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> Deref for Write<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> Deref for ReadWrite<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for Write<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct MapRead<'a, T>(MappedRwLockReadGuard<'a, T>);

impl<'a, T> MapRead<'a, T> {
    pub fn new<U, F>(lock: &'a Lock<U>, f: F) -> Self
    where
        F: FnOnce(&U) -> &T,
    {
        Self(RwLockReadGuard::map(lock.0.read(), f))
    }

    pub fn try_new<U, F>(lock: &'a Lock<U>, f: F) -> Option<Self>
    where
        F: FnOnce(&U) -> &T,
    {
        lock.0
            .try_read()
            .map(|guard| RwLockReadGuard::map(guard, f))
            .map(Self)
    }

    pub fn into_inner(self) -> MappedRwLockReadGuard<'a, T> {
        self.0
    }
}

impl<'a, T> Deref for MapRead<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: PartialEq> PartialEq for MapRead<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}

#[derive(Debug)]
pub struct MapWrite<'a, T>(MappedRwLockWriteGuard<'a, T>);

impl<'a, T> MapWrite<'a, T> {
    pub fn new<U, F>(lock: &'a Lock<U>, f: F) -> Self
    where
        F: FnOnce(&mut U) -> &mut T,
    {
        Self(RwLockWriteGuard::map(lock.0.write(), f))
    }

    pub fn try_new<U, F>(lock: &'a Lock<U>, f: F) -> Option<Self>
    where
        F: FnOnce(&mut U) -> &mut T,
    {
        lock.0
            .try_write()
            .map(|guard| RwLockWriteGuard::map(guard, f))
            .map(Self)
    }

    pub fn into_inner(self) -> MappedRwLockWriteGuard<'a, T> {
        self.0
    }
}

impl<'a, T> Deref for MapWrite<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for MapWrite<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T: PartialEq> PartialEq for MapWrite<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct SharedLock<T: ?Sized>(Arc<Lock<T>>);

impl<T> SharedLock<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(Lock::new(value)))
    }

    pub fn downgrade(&self) -> Weak<Lock<T>> {
        Arc::downgrade(&self.0)
    }

    pub fn read(&self) -> Read<'_, T> {
        Read::new(&self.0)
    }

    pub fn write(&self) -> Write<'_, T> {
        Write::new(&self.0)
    }

    pub fn read_write(&self) -> ReadWrite<'_, T> {
        ReadWrite::new(&self.0)
    }

    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }
}

impl<T> Clone for SharedLock<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Clone> From<T> for SharedLock<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Deref for SharedLock<T> {
    type Target = Lock<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub enum DeferResult {
    Completed,
    /// The operation was deferred to no sooner than the next `defer()` call.
    /// The contained [`AtomicBool`] will flip to `true` when the operation is completed.
    Deferred(Arc<AtomicBool>),
}

impl DeferResult {
    pub fn check_completed(&mut self) -> bool {
        match self {
            DeferResult::Completed => true,
            DeferResult::Deferred(listener) => {
                if listener.load(Ordering::Acquire) {
                    *self = DeferResult::Completed;
                    true
                } else {
                    false
                }
            }
        }
    }
}

pub type DeferredFn<T> = Box<dyn FnOnce(&mut T)>;

#[derive(Clone)]
pub struct DeferLock<T: ?Sized> {
    lock: SharedLock<T>,
    queue: SharedLock<VecDeque<(DeferredFn<T>, Arc<AtomicBool>)>>,
}

impl<T> DeferLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            lock: SharedLock::new(value),
            queue: SharedLock::new(VecDeque::new()),
        }
    }

    pub fn read(&self) -> Read<T> {
        self.lock.read()
    }

    /// Attempts to obtain a write lock on the inner `T`.
    ///
    /// If a write lock can NOT be obtained at this moment, `when_writable` will be pushed to a FIFO queue
    /// of operations to perform the next time `defer()` is called and the lock is writable.
    ///
    /// If a write lock CAN be obtained at this moment, the aforementioned queue is first flushed
    /// (applying all queued operations), then the operation defined by `when_writable` is performed.
    ///
    /// # Returns
    ///
    /// Returns [`DeferResult::Completed`] on successful write/flush.
    ///
    /// Returns [`DeferResult::Deferred`] if a write lock could not be obtained at this moment (and thus,
    /// `when_writable` was instead pushed to the queue).
    pub fn defer<F>(&self, when_writable: F) -> DeferResult
    where
        F: FnOnce(&mut T) + 'static,
    {
        if let Some(mut value) = self.lock.try_write() {
            // if we can write, drain the queue first
            while let Some((f, listener)) = self.queue.write().pop_front() {
                f(&mut *value);

                // notify the DeferResult's listener that the operation has completed
                listener.store(true, Ordering::Release);
            }
            // then apply the latest operation
            when_writable(&mut *value);

            DeferResult::Completed
        } else {
            // if we can't currently obtain a write lock, push the operation to the queue
            let listener = Arc::new(AtomicBool::new(false));
            self.queue
                .write()
                .push_back((Box::new(when_writable), listener.clone()));

            DeferResult::Deferred(listener)
        }
    }

    pub fn flush(&self) -> bool {
        if let Some(mut value) = self.lock.try_write() {
            while let Some((f, listener)) = self.queue.write().pop_front() {
                f(&mut *value);

                // notify the DeferResult's listener that the operation has completed
                listener.store(true, Ordering::Release);
            }

            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defer() {
        let lock = DeferLock::new(1);
        assert_eq!(*lock.read(), 1);

        let mut res = lock.defer(|lock| *lock += 1);
        assert!(res.check_completed());
        assert_eq!(*lock.read(), 2);

        let read = lock.read();
        let mut res = lock.defer(|lock| *lock += 1);
        assert!(!res.check_completed());
        assert_eq!(*read, 2);

        drop(read);
        assert!(lock.flush());

        assert!(res.check_completed());
        assert_eq!(*lock.read(), 3);
    }
}
