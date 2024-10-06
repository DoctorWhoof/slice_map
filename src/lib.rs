#![no_std]
extern crate alloc;
use alloc::vec::Vec;

use core::{marker::PhantomData, ops::Range};

mod storage;
use storage::*;

#[cfg(feature = "vec")]
mod vec;
#[cfg(feature = "vec")]
pub use vec::*;

#[cfg(feature = "array")]
mod array;
#[cfg(feature = "array")]
pub use array::*;

/// A generic container to store a single type of data into unevenly sized slices.
/// Can be iterated by slice or by items. You probably want to use SliceArray (for no_std)
/// or SliceVec instead, unless you want to provide your own container, in which case you
/// need to implement the Storage<T> trait.
#[derive(Debug, Default)]
pub struct SliceMap<S: Storage<T>, T> {
    storage: S,              // Generic storage
    slices: Vec<Range<u32>>, // Ranges that map to individual item slices
    _marker: PhantomData<T>,
}

impl<S, T> SliceMap<S, T>
where
    S: Storage<T>,
{
    /// Returns a new SliceMap containing the provided storage object.
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            slices: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Clears the SliceMap.
    pub fn clear(&mut self) {
        self.storage.reset();
        self.slices.clear();
    }

    /// How many items are contained in all slices.
    pub fn items_len(&self) -> usize {
        self.storage.len()
    }

    /// How many slices are contained in the SliceMap.
    pub fn slices_len(&self) -> usize {
        self.slices.len()
    }

    /// Adds a slice with all items from an iterator of owned T items.
    pub fn add_items<I>(&mut self, new_items: I) -> Result<(),()>
    where
        I: IntoIterator<Item = T>,
    {
        let start = self.storage.len() as u32;
        self.storage.extend_from_iter(new_items)?; // Extend the generic storage
        let end = self.storage.len() as u32;
        self.slices.push(start..end);
        Ok(())
    }

    /// Returns a slice with the desired range
    pub fn get_slice(&self, index: usize) -> &[T] {
        let range = &self.slices[index];
        self.storage.get_slice(Range {
            start: range.start as usize,
            end: range.end as usize,
        })
    }

    /// Returns an iterator for slices of items.
    pub fn iter_slices(&self) -> SliceIter<S, T> {
        SliceIter {
            section_map: self,
            index: 0,
        }
    }

    /// Returns an iterator for each individual item.
    pub fn iter_items(&self) -> impl Iterator<Item = &T> {
        self.storage.iter_items() // Returns an iterator over individual items in the storage
    }
}

/// Iterator for SliceMap that returns slices of items.
pub struct SliceIter<'a, S, T>
where
    S: Storage<T>,
{
    section_map: &'a SliceMap<S, T>,
    index: usize,
}

impl<'a, S, T> Iterator for SliceIter<'a, S, T>
where
    S: Storage<T>,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.section_map.slices.len() {
            None
        } else {
            let slice = self.section_map.get_slice(self.index);
            self.index += 1;
            Some(slice)
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{SliceMap, Storage, Vec};

    // Adds slices with numbers 1 to n, where n grows to max_slices
    #[allow(unused)]
    pub(crate) fn test_storage(s: &mut SliceMap<impl Storage<i32>, i32>, max_slices: usize) {
        let mut item_count = 1;
        let mut item_len = 0;
        for slice in 1..=max_slices {
            let values: Vec<i32> = (1..=item_count as i32).into_iter().collect();
            item_len += values.len();
            item_count += 1;
            s.add_items(values);
            assert_eq!(s.items_len(), item_len);
            assert_eq!(s.slices_len(), slice);
            // println!("{:?}", s);
        }
    }

    #[test]
    #[cfg(feature = "vec")]
    fn test_vec() {
        let v: Vec<i32> = Vec::default();
        let mut slicemap = SliceMap::new(v);
        test_storage(&mut slicemap, 10);
        // println!("{:?}", slicemap);
    }

    #[test]
    #[cfg(feature = "array")]
    fn test_array_vec() {
        use crate::array::ArrayVec;
        let arr: ArrayVec<i32, 100> = ArrayVec::default();
        let mut slicemap = SliceMap::new(arr);
        test_storage(&mut slicemap, 10);
        // println!("{:?}", slicemap);
    }
}
