#![no_std]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]

// Tests.
#[cfg(test)]
pub(crate) mod test;

// Main.
pub type ResultOrStr<T> = Result<T, &'static str>;
pub type Slice = core::ops::Range<u32>;

use slotmap::{basic::Values, SlotMap};
use core::{marker::PhantomData, ops::Range};

extern crate alloc;
use alloc::vec::Vec;

slotmap::new_key_type! {
    /// A unique key generated every time you insert a new slice with [SliceMap::add_items()].
    pub struct SliceKey;
}

/// A generic container to store a single type of data into unevenly sized slices.
/// Can be iterated by slice or by items. You probably want to use SliceArray (for _no_std_)
/// or SliceVec instead, unless you want to provide your own container, in which case you
/// need to implement the [Storage] trait.
#[derive(Default, Debug)]
pub struct SliceMap<V>{
    pub(crate) items: Vec<V>, // Generic items
    pub(crate) slices: SlotMap<SliceKey, Slice>,  // Ranges that map to individual item slices
    _marker_values: PhantomData<V>,
}

impl<V> SliceMap<V> {
    /// Returns a new SliceMap containing the provided items object.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            slices: SlotMap::with_key(),
            _marker_values: PhantomData,
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
        let start: u32 = self
            .items
            .len()
            .try_into()
            .unwrap();
        // Extend the generic items
        self.items.extend(new_items);
        let end: u32 = self
            .items
            .len()
            .try_into()
            .unwrap();
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

    /// Returns an iterator for each individual item.
    pub fn iter_items(&self) -> impl Iterator<Item = &V> {
        self.items.iter() // Returns an iterator over individual items in the items
    }

    /// Removes a slice by key. Warning: Will cause all items to "shift" to occupy the removed space,
    /// and all slices will be updated with the new indices.
    pub fn remove_slice(&mut self, key: SliceKey) -> Option<Slice> {
        let removed_slice = self.slices.remove(key)?;

        // Remove the items in the range from items
        self.items.drain(removed_slice.start as usize .. removed_slice.end as usize);

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

/// Iterator for SliceMap that returns slices of items.
pub struct SliceIter<'a, V> {
    slice_map: &'a SliceMap<V>,
    slices: Values<'a, SliceKey, Slice>,
}

impl<'a, V> Iterator for SliceIter<'a, V> {
    type Item = &'a [V];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(slice) = self.slices.next(){
            self.slice_map.items.get(slice.start as usize .. slice.end as usize)
        } else {
            None
        }
    }
}
