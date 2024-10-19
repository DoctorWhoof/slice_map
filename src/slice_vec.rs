use crate::{Slice, SliceIter, SliceMap, Storage, StrResult};
use core::ops::Range;

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

// Implement the Storage trait for Vec<T>.
impl<T> Storage<T> for Vec<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.clear();
    }

    #[inline(always)]
    fn get_slice(&self, range: Range<usize>) -> Option<&[T]> {
        self.get(range)
    }

    #[inline(always)]
    fn items(&self) -> core::slice::Iter<T> {
        self.iter()
    }

    #[inline(always)]
    fn extend_from_iter<I: IntoIterator<Item = T>>(&mut self, iter: I) -> StrResult {
        self.extend(iter);
        Ok(())
    }

    #[inline(always)]
    fn get_item(&self, index: impl Into<usize>) -> Option<&T> {
        let index: usize = index.into();
        self.get(index)
    }

    #[inline(always)]
    fn push_item(&mut self, item: T) -> StrResult {
        self.push(item);
        Ok(())
    }

    #[inline(always)]
    fn remove(&mut self, index: impl Into<usize>) -> Option<T> {
        let i:usize = index.into();
        if i < self.len(){
            Some(self.remove(i))
        } else {
            None
        }
    }

    #[inline(always)]
    fn drain(&mut self, range: impl core::ops::RangeBounds<usize>) {
        self.drain(range);
    }

    #[inline(always)]
    fn items_mut(&mut self) -> core::slice::IterMut<T> {
        self.iter_mut()
    }
}
