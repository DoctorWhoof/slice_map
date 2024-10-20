use crate::{Slice, SliceIter, SliceMap, Storage, StrResult};
use core::ops::Range;
use slotmap::SlotMap;

extern crate alloc;
use alloc::vec::Vec;

/// A Container to store a single type of data into unevenly sized slices, backed by
/// Vecs. Can be iterated by slice or by items.
#[derive(Default, Debug)]
pub struct SliceSlots<K, V>
where
    K: slotmap::Key,
    V: Default,
{
    pub(crate) storage: SliceMap<V, Vec<V>, SlotMap<K,Slice>>,
}

/// Requires "vec" feature. A SliceMap that uses vecs for storage.
impl<K, V> SliceSlots<K,V>
where
    K: slotmap::Key,
    V: Default,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            storage: SliceMap::new(Vec::new(), SlotMap::with_key()),
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.storage.clear();
    }

    #[inline(always)]
    pub fn items(&self) -> &Vec<V> {
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
        ITER: IntoIterator<Item = V>,
    {
        self.storage.add_items(new_items)
    }

    #[inline(always)]
    pub fn get_slice(&self, index: usize) -> Option<&[V]> {
        self.storage.get_slice(index)
    }

    #[inline(always)]
    pub fn iter_slices(&self) -> SliceIter<V, Vec<V>, SlotMap<K,Slice>> {
        self.storage.iter_slices()
    }

    #[inline(always)]
    pub fn iter_items(&self) -> impl Iterator<Item = &V> {
        self.storage.iter_items()
    }

    #[inline(always)]
    pub fn remove_slice(&mut self, index:usize) {
        self.storage.remove_slice(index);
    }
}

// Implement the Storage trait for Vec<V>.
impl<K,V> Storage<V> for SlotMap<K,V>
where
    K: slotmap::Key,
    V: Default,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.clear();
    }

    #[inline(always)]
    fn get_slice(&self, range: Range<usize>) -> Option<&[V]> {
        // Getting values by range does not apply to SlotMap<K, Slice>
        None
    }

    #[inline(always)]
    fn items(&self) -> core::slice::Iter<V> {
        self.values()
    }

    #[inline(always)]
    fn extend_from_iter<I: IntoIterator<Item = V>>(&mut self, iter: I) -> StrResult {
        // self.extend(iter);
        Ok(())
    }

    #[inline(always)]
    fn get_item(&self, index: impl Into<usize>) -> Option<&V> {
        let index: usize = index.into();
        self.get(index)
    }

    #[inline(always)]
    fn push_item(&mut self, item: V) -> StrResult {
        self.push(item);
        Ok(())
    }

    #[inline(always)]
    fn remove(&mut self, index: impl Into<usize>) -> Option<K,V> {
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
    fn items_mut(&mut self) -> core::slice::IterMut<K,V> {
        self.iter_mut()
    }
}

// Implement the Storage trait for Vec<V>.
impl<K,V> Storage<V> for SecondaryMap<K,V>
where
    K: slotmap::Key,
    V: Default,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.clear();
    }

    #[inline(always)]
    fn get_slice(&self, range: Range<usize>) -> Option<&[V]> {
        self.get(range)
    }

    #[inline(always)]
    fn items(&self) -> core::slice::Iter<K,V> {
        self.iter()
    }

    #[inline(always)]
    fn extend_from_iter<I: IntoIterator<Item = V>>(&mut self, iter: I) -> StrResult {
        self.extend(iter);
        Ok(())
    }

    #[inline(always)]
    fn get_item(&self, index: impl Into<usize>) -> Option<&V> {
        let index: usize = index.into();
        self.get(index)
    }

    #[inline(always)]
    fn push_item(&mut self, item: V) -> StrResult {
        self.push(item);
        Ok(())
    }

    #[inline(always)]
    fn remove(&mut self, index: impl Into<usize>) -> Option<K,V> {
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
    fn items_mut(&mut self) -> core::slice::IterMut<K,V> {
        self.iter_mut()
    }
}
