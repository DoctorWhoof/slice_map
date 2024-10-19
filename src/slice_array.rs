use crate::{array_vec::ArrayVec, Slice, SliceIter, SliceMap, StrResult};

/// Requires "array" feature. A Container to store a single type of data into unevenly sized slices,
/// backed by arrays and const generics. Can be iterated by slice or by items.
#[derive(Default, Debug)]
pub struct SliceArray<T, const T_LEN: usize, const SLICE_LEN: usize>
where
    T: Default,
{
    pub(crate) storage: SliceMap<T, ArrayVec<T, T_LEN>, ArrayVec<Slice, SLICE_LEN>>,
}

impl<T, const T_LEN: usize, const SLICE_LEN: usize> SliceArray<T, T_LEN, SLICE_LEN>
where
    T: Default,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            storage: SliceMap::new(ArrayVec::default(), ArrayVec::default()),
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.storage.clear();
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
    pub fn iter_slices(&self) -> SliceIter<T, ArrayVec<T, T_LEN>, ArrayVec<Slice, SLICE_LEN>> {
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
