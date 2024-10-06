use crate::{Slice, SliceMap, Storage};
extern crate alloc;
use alloc::vec::Vec;

// Adds slices with numbers 1 to n, where n grows to max_slices
#[allow(unused)]
pub(crate) fn test_storage<I, S>(s: &mut SliceMap<i32, I, S>, max_slices: usize)
where
    I: Storage<i32>,
    S: Storage<Slice>,
{
    let mut item_count = 1;
    let mut item_len = 0;
    for slice in 1..=max_slices {
        let values: Vec<i32> = (1..=item_count as i32).into_iter().collect();
        item_len += values.len();
        item_count += 1;
        s.add_items(values);
        assert_eq!(s.items_len(), item_len);
        assert_eq!(s.slices_len(), slice);
        // println!("{:?}", s);
    }
}

#[test]
#[cfg(feature = "vec")]
fn test_vec() {
    let values: Vec<i32> = Vec::default();
    let ranges: Vec<Slice> = Vec::default();
    let mut slicemap = SliceMap::new(values, ranges);
    test_storage(&mut slicemap, 10);
}

#[test]
#[cfg(feature = "vec")]
fn test_vec_default() {
    use crate::SliceVec;
    let mut slicemap = SliceVec::default();
    test_storage(&mut slicemap, 10);
}

#[test]
#[cfg(feature = "array")]
fn test_array_vec() {
    use crate::array::ArrayVec;
    let values: ArrayVec<i32, 100> = ArrayVec::default();
    let ranges: ArrayVec<Slice, 10> = ArrayVec::default();
    let mut slicemap = SliceMap::new(values, ranges);
    test_storage(&mut slicemap, 10);
}

#[test]
#[cfg(feature = "array")]
fn test_array_default() {
    use crate::ArrayVec;
    let mut slicemap = SliceMap::<i32, ArrayVec<i32, 100>, ArrayVec<Slice, 10>>::default();
    test_storage(&mut slicemap, 10);
}
