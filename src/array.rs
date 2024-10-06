use crate::{SliceMap, Storage};
use core::ops::Range;

#[derive(Debug)]
pub struct ArrayVec<T, const ITEMS: usize> {
    data: [T; ITEMS],
    head: usize,
}

impl<T, const ITEMS: usize> Default for ArrayVec<T, ITEMS>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            data: core::array::from_fn(|_| Default::default()),
            head: 0,
        }
    }
}

impl<T, const ITEMS: usize> ArrayVec<T, ITEMS> {
    pub fn clear(&mut self) {
        self.head = 0;
    }

    pub fn push(&mut self, item: T) {
        if self.head >= ITEMS {
            return;
        }
        self.data[self.head] = item;
        self.head += 1;
    }

    /// Extends the ArrayVec with items from the iterator.
    pub fn extend<I>(&mut self, source: I)
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = source.into_iter();
        while let Some(item) = iter.next() {
            self.push(item);
        }
    }
}

/// Implement the Storage trait for Vec<T>
impl<T, const ITEMS: usize> Storage<T> for ArrayVec<T, ITEMS> {
    fn len(&self) -> usize {
        self.head
    }

    fn reset(&mut self) {
        self.clear();
    }

    fn get_slice(&self, range: Range<usize>) -> &[T] {
        &self.data[range]
    }

    fn iter_items(&self) -> core::slice::Iter<T> {
        self.data.iter()
    }

    fn extend_from_iter<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.extend(iter);
    }
}

/// A SliceMap that uses a fixed size ArrayVec for storage. You must specify the capacity
/// for both the number of items and the number of slices.
pub type SliceArray<T, const ITEMS: usize, const SLICES: usize> =
    SliceMap<ArrayVec<T, ITEMS>, T>;

impl<T, const ITEMS: usize> SliceMap<ArrayVec<T, ITEMS>, T>
where
    T: Default,
{
    pub fn new_with_arrayvec() -> Self {
        Self::new(ArrayVec::default())
    }
}
