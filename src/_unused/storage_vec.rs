use core::ops::Range;
use crate::{Storage, StrResult};

extern crate alloc;
use alloc::vec::Vec;

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
