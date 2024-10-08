use crate::{Storage, StrResult};
use core::ops::Range;

/// Requires "array" feature. A very simple "vec-like" container with fixed size.
/// Pushing items beyond its capacity will do nothing aside from returning an error.
#[derive(Debug)]
pub struct ArrayVec<T, const ITEM_COUNT: usize> {
    data: [T; ITEM_COUNT],
    head: usize,
}

impl<T, const ITEM_COUNT: usize> Default for ArrayVec<T, ITEM_COUNT>
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

impl<T, const ITEM_COUNT: usize> ArrayVec<T, ITEM_COUNT> {
    pub fn clear(&mut self) {
        self.head = 0;
    }

    pub fn get(&self, index: impl Into<usize>) -> Option<&T> {
        let index: usize = index.into();
        self.data.get(index)
    }

    pub fn push(&mut self, item: T) -> StrResult {
        if self.head >= ITEM_COUNT {
            return Err("ArrayVec capacity exceeded");
        }
        self.data[self.head] = item;
        self.head += 1;
        Ok(())
    }

    /// Extends the ArrayVec with items from the iterator.
    pub fn extend<I>(&mut self, source: I) -> StrResult
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = source.into_iter();
        while let Some(item) = iter.next() {
            self.push(item)?;
        }
        Ok(())
    }
}

// Implement the Storage trait for Vec<T>
impl<T, const ITEM_COUNT: usize> Storage<T> for ArrayVec<T, ITEM_COUNT> {
    fn len(&self) -> usize {
        self.head
    }

    fn reset(&mut self) {
        self.clear();
    }

    fn get_slice(&self, range: Range<usize>) -> Option<&[T]> {
        self.data.get(range)
    }

    fn iter_items(&self) -> core::slice::Iter<T> {
        self.data.iter()
    }

    fn get_item(&self, index: impl Into<usize>) -> Option<&T> {
        self.get(index)
    }

    fn push_item(&mut self, item: T) -> StrResult {
        self.push(item)?;
        Ok(())
    }

    fn extend_from_iter<I: IntoIterator<Item = T>>(&mut self, iter: I) -> StrResult {
        self.extend(iter)?;
        Ok(())
    }
}
