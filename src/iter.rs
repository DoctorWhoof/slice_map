use slotmap::{basic::Values, Key};

use crate::{SecSliceMap, Slice, SliceKey, SliceMap};


/// Iterator for SliceMap that returns slices of items.
pub struct SliceIter<'a, V> {
    pub slice_map: &'a SliceMap<V>,
    pub slices: Values<'a, SliceKey, Slice>,
}

impl<'a, V> Iterator for SliceIter<'a, V> {
    type Item = &'a [V];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(slice) = self.slices.next() {
            self.slice_map
                .items
                .get(slice.start as usize..slice.end as usize)
        } else {
            None
        }
    }
}

/// Iterator for SliceMap that returns slices of items along with their keys.
pub struct KeySliceIter<'a, V> {
    pub slice_map: &'a SliceMap<V>,
    pub slices: slotmap::basic::Iter<'a, SliceKey, Slice>,
}

impl<'a, V> Iterator for KeySliceIter<'a, V> {
    type Item = (SliceKey, &'a [V]);

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


/// Iterator for SecSliceMap that returns slices of items.
pub struct SecSliceIter<'a, K, V>
where K:Key
{
    pub slice_map: &'a SecSliceMap<K, V>,
    pub slices: slotmap::secondary::Values<'a, K, Slice>,
}

impl<'a, K, V> Iterator for SecSliceIter<'a, K, V>
where K:Key
{
    type Item = &'a [V];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(slice) = self.slices.next() {
            self.slice_map
                .items
                .get(slice.start as usize..slice.end as usize)
        } else {
            None
        }
    }
}


/// Iterator for SecSliceMap that returns slices of items along with their keys.
pub struct SecKeySliceIter<'a, K, V>
where K:Key
{
    pub slice_map: &'a SecSliceMap<K, V>,
    pub slices: slotmap::secondary::Iter<'a, K, Slice>,
}

impl<'a, K, V> Iterator for SecKeySliceIter<'a, K, V>
where K:Key
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
