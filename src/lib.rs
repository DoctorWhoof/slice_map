// #![no_std]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]

// Tests.
#[cfg(test)]
pub(crate) mod test;

// Modules
mod traits;
pub use traits::*;

mod iter;
pub use iter::*;

use core::{marker::PhantomData, ops::Range};
use slotmap::{Key, SecondaryMap, SlotMap, SparseSecondaryMap};

extern crate alloc;
use alloc::vec::Vec;

/// This generic SliceMap needs to be provided a Key type, a Value type and a Storage type.
/// Use [SlotSliceMap] and [SecSliceMap] for storage using SlotMap and SecondarySlotMap, respectively.
#[derive(Default, Debug, Clone)]
pub struct SliceMap<K, V, S>
where
    K: Key,
    S: SliceStorage<K, Range<u32>>,
{
    pub(crate) items: Vec<V>, // Generic items
    pub(crate) slices: S,     // Generic slice storage
    type_key: PhantomData<K>,
}

impl<K, V, S> SliceMap<K, V, S>
where
    K: Key,
    S: SliceStorage<K, Range<u32>> + Default,
{
    /// Returns a new SliceMap containing the provided items object.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            slices: S::default(),
            type_key: Default::default(),
        }
    }

    /// Returns a new SliceMap with the specified initial capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: Vec::with_capacity(cap),
            slices: S::default(),
            type_key: Default::default(),
        }
    }

    /// Clears the SliceMap.
    pub fn clear(&mut self) {
        self.items.clear();
        self.slices = S::default();
    }

    /// Returns a slice with all items in all slices.
    pub fn items(&self) -> &[V] {
        &self.items
    }

    /// How many items are contained in all slices.
    pub fn items_len(&self) -> usize {
        self.items.len()
    }

    /// True if no items
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// How many slices are contained in the SliceMap.
    pub fn slices_len(&self) -> usize {
        self.slices.iter().count()
    }

    /// Returns a slice with the desired range
    pub fn get_slice(&self, key: K) -> Option<&[V]> {
        let range = self.slices.get(key)?;
        self.items.get(range.start as usize..range.end as usize)
    }

    /// Returns an iterator for slices of items.
    pub fn iter_slices(&self) -> SliceIter<K, V, S> {
        SliceIter {
            slice_map: &self,
            slices: self.slices.values(),
            type_data: Default::default(),
        }
    }

    /// Returns an iterator for slices of items along with their keys.
    pub fn iter_keys_and_slices(&self) -> KeySliceIter<K, V, S> {
        KeySliceIter {
            slice_map: &self,
            slices: self.slices.iter(),
            type_data: Default::default(),
        }
    }

    /// Returns an iterator for each individual item.
    pub fn iter_items(&self) -> impl Iterator<Item = &V> {
        self.items.iter() // Returns an iterator over individual items in the items
    }

    /// Removes a slice by key. Warning: Will cause all items to "shift" to occupy the removed space,
    /// and all slices will be updated with the new indices.
    pub fn remove_slice(&mut self, key: K) -> Option<Range<u32>> {
        let removed_slice = self.slices.remove(key)?;

        // Remove the items in the range from items
        self.items
            .drain(removed_slice.start as usize..removed_slice.end as usize);

        // Adjust the slices of all subsequent slices
        let offset = removed_slice.end - removed_slice.start;
        for slice in self.slices.values_mut() {
            if slice.start >= removed_slice.end {
                slice.start = u32::try_from(slice.start - offset).expect("Index out of bounds");
                slice.end = u32::try_from(slice.end - offset).expect("Index out of bounds");
            }
        }

        Some(removed_slice)
    }
}

/// SliceMap that uses [slotmap::SlotMap] for range storage
pub type SlotSliceMap<K, V> = SliceMap<K, V, SlotMap<K, Range<u32>>>;

impl<K, V> SlotSliceMap<K, V>
where
    K: Key,
    V: Clone, // Clone is required to handle &V inputs
{
    /// Creates a new slice with all items from an iterator of owned or borrowed V items.
    /// Accepts arrays, slices, or any type that implements AsRef<[V]>.
    /// Will panic if the capacity of [u32::MAX] items is reached.
    pub fn add_items<ITEMS>(&mut self, new_items: ITEMS) -> K
    where
        ITEMS: AsRef<[V]>, // Accepts &[V], [V; LEN], or other AsRef<[V]> types
    {
        let start: u32 = self.items.len().try_into().unwrap();

        // Extend items with the cloned elements from the input slice
        self.items.extend(new_items.as_ref().iter().cloned());

        let end: u32 = self.items.len().try_into().unwrap();
        self.slices.insert(start..end)
    }
}

/// SliceMap that uses [slotmap::SecondaryMap] for range storage
pub type SecSliceMap<K, V> = SliceMap<K, V, SecondaryMap<K, Range<u32>>>;

impl<K, V> SecSliceMap<K, V>
where
    K: Key,
    V: Clone, // Clone is required to handle &V inputs
{
    /// Creates a new slice with all items from an iterable of owned or borrowed V items.
    /// Accepts arrays, slices, or any other AsRef<[V]> type.
    /// Will panic if the capacity of [u32::MAX] items is reached.
    pub fn add_items<ITEMS>(&mut self, key: K, new_items: ITEMS)
    where
        ITEMS: AsRef<[V]>, // Accepts &[V], [V; LEN], Vec<V>, or other AsRef<[V]> types
    {
        let start: u32 = self.items.len().try_into().unwrap();

        // Extend items with the cloned elements from the input slice
        self.items.extend(new_items.as_ref().iter().cloned());

        let end: u32 = self.items.len().try_into().unwrap();
        self.slices.insert(key, start..end);
    }
}

/// SliceMap that uses [slotmap::SparseSecondaryMap] for range storage
pub type SparseSliceMap<K, V> = SliceMap<K, V, SparseSecondaryMap<K, Range<u32>>>;

impl<K, V> SparseSliceMap<K, V>
where
    K: Key,
    V: Clone, // Clone is required to handle &V inputs
{
    /// Creates a new slice with all items from an iterable of owned or borrowed V items.
    /// Accepts arrays, slices, or any other AsRef<[V]> type.
    /// Will panic if the capacity of [u32::MAX] items is reached.
    pub fn add_items<ITEMS>(&mut self, key: K, new_items: ITEMS)
    where
        ITEMS: AsRef<[V]>, // Accepts &[V], [V; LEN], Vec<V>, or other AsRef<[V]> types
    {
        let start: u32 = self.items.len().try_into().unwrap();

        // Extend items with the cloned elements from the input slice
        self.items.extend(new_items.as_ref().iter().cloned());

        let end: u32 = self.items.len().try_into().unwrap();
        self.slices.insert(key, start..end);
    }
}
