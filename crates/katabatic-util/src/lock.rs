use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Weak},
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

    pub fn into_inner(self) -> Option<T> {
        Some(RwLock::into_inner(Arc::into_inner(self.0)?.0))
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
