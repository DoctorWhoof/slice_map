#![no_std]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]

// Modules.
#[cfg(test)]
pub(crate) mod test;

mod sec_slice_map;
pub use sec_slice_map::*;

pub mod iter;
use iter::*;

// Main.
use core::ops::Range;
use slotmap::SlotMap;

extern crate alloc;
use alloc::vec::Vec;

type Slice = core::ops::Range<u32>;

slotmap::new_key_type! {
    /// A unique key generated every time you insert a new slice with [SliceMap::add_items()].
    pub struct SliceKey;
}

/// A generic container to store a single type of data into unevenly sized slices.
/// Can be iterated by slice or by items.
#[derive(Default, Debug, Clone)]
pub struct SliceMap<V> {
    pub(crate) items: Vec<V>,                    // Generic items
    pub(crate) slices: SlotMap<SliceKey, Slice>, // Ranges that map to individual item slices
}

impl<V> SliceMap<V> {
    /// Returns a new SliceMap containing the provided items object.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            slices: SlotMap::with_key(),
        }
    }

    /// Returns a new SliceMap with the specified initial capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: Vec::with_capacity(cap),
            slices: SlotMap::with_key(),
        }
    }

    /// Clears the SliceMap.
    pub fn clear(&mut self) {
        self.items.clear();
        self.slices.clear();
    }

    /// Returns a slice with all items in all slices.
    pub fn items(&self) -> &[V] {
        &self.items
    }

    /// Creates a new slice with all items from an iterator of owned V items.
    /// Will panic if the capacity of [u32::MAX] items is reached.
    pub fn add_items<ITER>(&mut self, new_items: ITER) -> SliceKey
    where
        ITER: IntoIterator<Item = V>,
    {
        let start: u32 = self.items.len().try_into().unwrap();
        // Extend the generic items
        self.items.extend(new_items);
        let end: u32 = self.items.len().try_into().unwrap();
        self.slices.insert(start..end)
    }

    /// How many items are contained in all slices.
    pub fn items_len(&self) -> usize {
        self.items.len()
    }

    /// How many slices are contained in the SliceMap.
    pub fn slices_len(&self) -> usize {
        self.slices.len()
    }

    /// Returns a slice with the desired range
    pub fn get_slice(&self, key: SliceKey) -> Option<&[V]> {
        let range = self.slices.get(key)?;
        self.items.get(Range {
            start: range.start as usize,
            end: range.end as usize,
        })
    }

    /// Returns an iterator for slices of items.
    pub fn iter_slices(&self) -> SliceIter<V> {
        SliceIter {
            slice_map: &self,
            slices: self.slices.values(),
        }
    }

    /// Returns an iterator for slices of items along with their keys.
    pub fn iter_keys_and_slices(&self) -> KeySliceIter<V> {
        KeySliceIter {
            slice_map: &self,
            slices: self.slices.iter(),
        }
    }

    /// Returns an iterator for each individual item.
    pub fn iter_items(&self) -> impl Iterator<Item = &V> {
        self.items.iter() // Returns an iterator over individual items in the items
    }

    /// Removes a slice by key. Warning: Will cause all items to "shift" to occupy the removed space,
    /// and all slices will be updated with the new indices.
    pub fn remove_slice(&mut self, key: SliceKey) -> Option<Slice> {
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
