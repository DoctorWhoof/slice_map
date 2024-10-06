use crate::{Storage, Slice, StrResult};
use core::{marker::PhantomData, ops::Range};

/// A generic container to store a single type of data into unevenly sized slices.
/// Can be iterated by slice or by items. You probably want to use SliceArray (for no_std)
/// or SliceVec instead, unless you want to provide your own container, in which case you
/// need to implement the Storage<T> trait.
#[derive(Debug, Default)]
pub struct SliceMap<T, I, S>
where
    T: Default,
    I: Storage<T>,
    S: Storage<Slice>,
{
    storage: I, // Generic storage
    slices: S,  // Ranges that map to individual item slices
    _marker: PhantomData<T>,
}

impl<T, I, S> SliceMap<T, I, S>
where
    T: Default,
    I: Storage<T>,
    S: Storage<Slice>,
{
    /// Returns a new SliceMap containing the provided storage object.
    pub fn new(storage: I, slices: S) -> Self {
        Self {
            storage,
            slices,
            _marker: PhantomData,
        }
    }

    /// Clears the SliceMap.
    pub fn clear(&mut self) {
        self.storage.reset();
        self.slices.reset();
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
    pub fn add_items<ITER>(&mut self, new_items: ITER) -> StrResult
    where
        ITER: IntoIterator<Item = T>,
    {
        let start = self.storage.len();
        self.storage.extend_from_iter(new_items)?; // Extend the generic storage
        let end = self.storage.len();
        self.slices
            .push_item(start.try_into().unwrap()..end.try_into().unwrap())?;
        Ok(())
    }

    /// Returns a slice with the desired range
    pub fn get_slice(&self, index: usize) -> Option<&[T]> {
        let range = self.slices.get_item(index)?;
        self.storage.get_slice(Range {
            start: range.start as usize,
            end: range.end as usize,
        })
    }

    /// Returns an iterator for slices of items.
    pub fn iter_slices(&self) -> SliceIter<T, I, S> {
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
pub struct SliceIter<'a, T, I, S>
where
    T: Default,
    I: Storage<T>,
    S: Storage<Slice>,
{
    section_map: &'a SliceMap<T, I, S>,
    index: usize,
}

impl<'a, T, I, S> Iterator for SliceIter<'a, T, I, S>
where
    T: Default,
    I: Storage<T>,
    S: Storage<Slice>,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.section_map.slices.len() {
            None
        } else {
            let slice = self.section_map.get_slice(self.index)?;
            self.index += 1;
            Some(slice)
        }
    }
}
