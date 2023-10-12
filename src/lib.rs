//! A crate with a typed wrapper for [`Slab`].

#![warn(missing_docs)]

pub use slab::Slab;

use derive_more::{Deref, DerefMut};
use std::marker::PhantomData;

/// Pre-allocated storage for a uniform data type with indeces
/// converted from and into `usize`.
#[derive(Debug, Deref, DerefMut)]
pub struct TypedSlab<K, V> {
    #[deref]
    #[deref_mut]
    slab: Slab<V>,
    _key: PhantomData<K>,
}

impl<K, V> Default for TypedSlab<K, V> {
    fn default() -> Self {
        Self {
            slab: Slab::default(),
            _key: PhantomData,
        }
    }
}

impl<K, V> TypedSlab<K, V>
where
    K: From<usize> + Into<usize>,
{
    /// Construct a new, empty `TypedSlab`.
    pub fn new() -> Self {
        Self {
            slab: Slab::new(),
            _key: PhantomData,
        }
    }

    /// Insert a value in the slab, returning key assigned to the value.
    pub fn insert(&mut self, value: V) -> K {
        let idx = self.slab.insert(value);
        K::from(idx)
    }

    /// Insert a value in the slab, returning key assigned and a reference
    /// to the stored value.
    pub fn insert_entry(&mut self, value: V) -> (K, &mut V) {
        let entry = self.slab.vacant_entry();
        let idx = entry.key();
        let value_mut = entry.insert(value);
        (K::from(idx), value_mut)
    }

    /// Remove and return the value associated with the given key.
    /// The key is then released and may be associated with future stored values.
    ///
    /// # Panics
    ///
    /// Panics if key is not associated with a value.
    ///
    pub fn remove(&mut self, key: K) -> Option<V> {
        let idx = key.into();
        if self.slab.contains(idx) {
            let value = self.slab.remove(idx);
            Some(value)
        } else {
            None
        }
    }

    /// Return a reference to the value associated with the given key.
    /// If the given key is not associated with a value, then `None` is returned.
    pub fn get(&self, key: K) -> Option<&V> {
        let idx = key.into();
        self.slab.get(idx)
    }

    /// Return a mutable reference to the value associated with the given key.
    /// If the given key is not associated with a value, then `None` is returned.
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let idx = key.into();
        self.slab.get_mut(idx)
    }

    /// Return true if there are no values stored in the slab.
    pub fn is_empty(&self) -> bool {
        self.slab.is_empty()
    }

    /// Return an iterator over the slab.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (K, &V)> {
        self.slab.iter().map(|(idx, v)| (idx.into(), v))
    }

    /// Return an iterator that allows modifying each value.
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = (K, &mut V)> {
        self.slab.iter_mut().map(|(idx, v)| (idx.into(), v))
    }

    /// Return an iterator with references to values.
    pub fn values(&self) -> impl DoubleEndedIterator<Item = &V> {
        self.slab.iter().map(|(_, v)| v)
    }

    /// Return a draining iterator that removes all elements from the slab
    /// and yields the removed items.
    pub fn drain(&mut self) -> impl DoubleEndedIterator<Item = V> + '_ {
        self.slab.drain()
    }

    /// Return a number of stored values.
    pub fn len(&self) -> usize {
        self.slab.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rev() {
        let slab: TypedSlab<usize, ()> = TypedSlab::new();
        let _iter = slab.iter().rev();
    }
}
