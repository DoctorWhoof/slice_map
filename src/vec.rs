extern crate alloc;
use alloc::vec::Vec;
use core::ops::Range;
use crate::{Slice, StrResult};

use super::{SliceMap, Storage};

// Implement the Storage trait for Vec<T>.
impl<T> Storage<T> for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn reset(&mut self) {
        self.clear();
    }

    fn get_slice(&self, range: Range<usize>) -> Option<&[T]> {
        self.get(range)
    }

    fn iter_items(&self) -> core::slice::Iter<T> {
        self.iter()
    }

    fn extend_from_iter<I: IntoIterator<Item = T>>(&mut self, iter: I) -> StrResult {
        self.extend(iter);
        Ok(())
    }

    fn get_item(&self, index:impl Into<usize>) -> Option<&T> {
        let index:usize = index.into();
        self.get(index)
    }

    fn push_item(&mut self, item: T) -> StrResult  {
        self.push(item);
        Ok(())
    }
}


/// Requires "vec" feature. A SliceMap that uses vecs for storage.
pub type SliceVec<T> = SliceMap<T, Vec<T>, Vec<Slice>>;

impl<T> SliceMap<T, Vec<T>, Vec<Slice>>
where T:Default
{
    pub fn new_with_vec() -> Self {
        Self::new(Vec::new(), Vec::new())
    }
}
