use core::marker::PhantomData;
use alloc::boxed::Box;
use slotmap::Key;
use core::ops::Range;

use crate::SliceStorage;

/// Iterator for `SliceMap` that returns slices of items.
pub struct SliceIter<'a, K, V, S>
where
    K: Key,
    S: SliceStorage<K, Range<u32>>,
    V: 'a,
{
    pub slice_map: &'a crate::SliceMap<K, V, S>, // Borrowed reference to the SliceMap
    pub slices: Box<dyn Iterator<Item = &'a Range<u32>> + 'a>, // Generic iterator over slice ranges
    pub type_data: PhantomData<V>,
}

impl<'a, K, V, S> Iterator for SliceIter<'a, K, V, S>
where
    K: Key,
    S: SliceStorage<K, Range<u32>>,
    V: 'a,
{
    type Item = &'a [V];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(slice) = self.slices.next() {
            self.slice_map.items.get(slice.start as usize..slice.end as usize)
        } else {
            None
        }
    }
}

/// Iterator for `SliceMap` that returns slices of items along with their keys.
pub struct KeySliceIter<'a, K, V, S>
where
    K: Key,
    S: SliceStorage<K, Range<u32>>,
    V: 'a,
{
    pub slice_map: &'a crate::SliceMap<K, V, S>, // Borrowed reference to the SliceMap
    pub slices: Box<dyn Iterator<Item = (K, &'a Range<u32>)> + 'a>, // Generic iterator over key-value pairs
    pub type_data: PhantomData<V>,
}

impl<'a, K, V, S> Iterator for KeySliceIter<'a, K, V, S>
where
    K: Key,
    S: SliceStorage<K, Range<u32>>,
    V: 'a,
{
    type Item = (K, &'a [V]);

    fn next(&mut self) -> Option<Self::Item> {
        // Get the next key and slice pair
        if let Some((key, slice)) = self.slices.next() {
            // Attempt to retrieve the slice of items
            self.slice_map
                .items
                .get(slice.start as usize..slice.end as usize)
                .map(|item_slice| (key, item_slice))
        } else {
            None
        }
    }
}
