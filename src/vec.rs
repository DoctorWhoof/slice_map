use core::ops::Range;
use alloc::vec::Vec;
use super::{SliceMap, Storage};

/// Implement the Storage trait for Vec<T>
impl<T> Storage<T> for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn reset(&mut self) {
        self.clear();
    }

    fn get_slice(&self, range: Range<usize>) -> &[T] {
        &self[range]
    }

    fn iter_items(&self) -> core::slice::Iter<T> {
        self.iter()
    }

    fn extend_from_iter<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(),()> {
        self.extend(iter);
        Ok(())
    }
}


/// Requires "vec" feature. A SliceMap that uses a vec for storage
pub type SliceVec<T> = SliceMap<Vec<T>, T>;

impl<T> SliceMap<Vec<T>, T> {
    pub fn new_with_vec() -> Self {
        Self::new(Vec::new())
    }
}
