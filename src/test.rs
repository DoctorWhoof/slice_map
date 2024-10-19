use crate::{Slice, SliceMap, Storage};
extern crate alloc;
use alloc::vec::Vec;

// Adds slices with numbers 1 to n, where n grows to max_slices
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
        s.add_items(values).ok();
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
    test_storage(&mut slicemap.storage, 10);
}


#[test]
#[cfg(feature = "vec")]
fn test_remove(){
    use crate::SliceVec;
    let mut slicevec = SliceVec::default();

    slicevec.add_items([1, 2, 3, 4, 5]).ok();
    slicevec.add_items([6, 7]).ok();
    slicevec.add_items([8, 9, 10]).ok();

    // Remove
    slicevec.remove_slice(1);

    // Iterating over slices
    assert_eq!(slicevec.slices_len(), 2);
    let mut slices = slicevec.iter_slices();
    assert_eq!(slices.next().unwrap(), [1, 2, 3, 4, 5]);
    assert_eq!(slices.next().unwrap(), [8, 9, 10]);
    assert_eq!(slices.next(), None);

    // Iterating over all items
    let mut value = 1;
    for (i, item) in slicevec.iter_items().enumerate(){
        if i < 5 {
            assert_eq!(value, *item);
        } else {
            assert_eq!(value + 2, *item);
        }
        value += 1
    }

    // Remove and test again
    slicevec.remove_slice(0);
    assert_eq!(slicevec.slices_len(), 1);
    let mut slices = slicevec.iter_slices();
    assert_eq!(slices.next().unwrap(), [8, 9, 10]);
    assert_eq!(slices.next(), None);
    let mut value = 8;
    for item in slicevec.iter_items(){
        assert_eq!(value, *item);
        value += 1
    }

    // Empty
    slicevec.remove_slice(0);
    assert_eq!(slicevec.items_len(), 0);
    assert_eq!(slicevec.slices_len(), 0);

}


// #[test]
// #[cfg(feature = "array")]
// fn test_array_vec() {
//     use crate::ArrayVec;
//     let values: ArrayVec<i32, 100> = ArrayVec::default();
//     let ranges: ArrayVec<Slice, 10> = ArrayVec::default();
//     let mut slicemap = SliceMap::new(values, ranges);
//     test_storage(&mut slicemap, 10);
// }

// #[test]
// #[cfg(feature = "array")]
// fn test_array_default() {
//     use crate::ArrayVec;
//     let mut slicemap = SliceMap::<i32, ArrayVec<i32, 100>, ArrayVec<Slice, 10>>::default();
//     test_storage(&mut slicemap, 10);
// }
