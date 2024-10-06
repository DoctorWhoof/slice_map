use core::ops::Range;

/// The required trait for any kind of storage used in a SliceMap.
pub trait Storage<T> {
    fn len(&self) -> usize;
    fn reset(&mut self);
    fn get_slice(&self, range: Range<usize>) -> &[T];
    fn iter_items(&self) -> core::slice::Iter<T>;
    fn extend_from_iter<I: IntoIterator<Item = T>>(&mut self, iter: I);
}
