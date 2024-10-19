use core::ops::{Range, RangeBounds};

use crate::StrResult;

/// The required trait for any kind of storage used in a SliceMap.
pub trait Storage<T> {
    fn len(&self) -> usize;
    fn reset(&mut self);
    fn items(&self) -> core::slice::Iter<T>;
    fn items_mut(&mut self) -> core::slice::IterMut<T>;
    fn get_item(&self, index: impl Into<usize>) -> Option<&T>;
    fn get_slice(&self, range: Range<usize>) -> Option<&[T]>;
    fn push_item(&mut self, item: T) -> StrResult;
    fn extend_from_iter<I: IntoIterator<Item = T>>(&mut self, iter: I) -> StrResult;
    fn remove(&mut self, index: impl Into<usize>) -> Option<T>;
    fn drain(&mut self, range: impl RangeBounds<usize>);
}
