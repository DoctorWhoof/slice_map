use crate::{Slice, SliceIter, SliceMap, StrResult};

extern crate alloc;
use alloc::vec::Vec;


/// A Container to store a single type of data into unevenly sized slices, backed by
/// Vecs. Can be iterated by slice or by items.
#[derive(Default, Debug)]
pub struct SliceVec<T>
where
    T: Default,
{
    pub(crate) storage: SliceMap<T, Vec<T>, Vec<Slice>>,
}

/// Requires "vec" feature. A SliceMap that uses vecs for storage.
impl<T> SliceVec<T>
where
    T: Default,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            storage: SliceMap::new(Vec::new(), Vec::new()),
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.storage.clear();
    }

    #[inline(always)]
    pub fn items(&self) -> &Vec<T> {
        &self.storage.items
    }

    #[inline(always)]
    pub fn items_len(&self) -> usize {
        self.storage.items_len()
    }

    #[inline(always)]
    pub fn slices_len(&self) -> usize {
        self.storage.slices_len()
    }

    #[inline(always)]
    pub fn add_items<ITER>(&mut self, new_items: ITER) -> StrResult
    where
        ITER: IntoIterator<Item = T>,
    {
        self.storage.add_items(new_items)
    }

    #[inline(always)]
    pub fn get_slice(&self, index: usize) -> Option<&[T]> {
        self.storage.get_slice(index)
    }

    #[inline(always)]
    pub fn iter_slices(&self) -> SliceIter<T, Vec<T>, Vec<Slice>> {
        self.storage.iter_slices()
    }

    #[inline(always)]
    pub fn iter_items(&self) -> impl Iterator<Item = &T> {
        self.storage.iter_items()
    }

    #[inline(always)]
    pub fn remove_slice(&mut self, index:usize) {
        self.storage.remove_slice(index);
    }
}
