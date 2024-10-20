use crate::SliceMap;

extern crate alloc;
use alloc::vec::Vec;

#[test]
fn test_basic() {
    let mut slicemap = SliceMap::new();
    let max_slices = 10;
    let mut item_count = 1;
    let mut item_len = 0;
    for slice in 1..=max_slices {
        let values: Vec<i32> = (1..=item_count as i32).into_iter().collect();
        item_len += values.len();
        item_count += 1;
        slicemap.add_items(values);
        assert_eq!(slicemap.items_len(), item_len);
        assert_eq!(slicemap.slices_len(), slice);
        // println!("{:?}", s);
    }
}

#[test]
fn test_remove() {
    let mut slicevec = SliceMap::default();

    let a = slicevec.add_items([1, 2, 3, 4, 5]);
    let b = slicevec.add_items([6, 7]);
    let c = slicevec.add_items([8, 9, 10]);
    assert_eq!(slicevec.items_len(), 10);
    assert_eq!(slicevec.slices_len(), 3);

    // Remove
    slicevec.remove_slice(b);

    // Iterating over slices
    assert_eq!(slicevec.slices_len(), 2);
    let mut slices = slicevec.iter_slices();
    assert_eq!(slices.next().unwrap(), [1, 2, 3, 4, 5]);
    assert_eq!(slices.next().unwrap(), [8, 9, 10]);
    assert_eq!(slices.next(), None);

    // Iterating over all items
    let mut value = 1;
    for (i, item) in slicevec.iter_items().enumerate() {
        if i < 5 {
            assert_eq!(value, *item);
        } else {
            assert_eq!(value + 2, *item);
        }
        value += 1
    }

    // Remove and test again
    slicevec.remove_slice(a);
    assert_eq!(slicevec.slices_len(), 1);
    let mut slices = slicevec.iter_slices();
    assert_eq!(slices.next().unwrap(), [8, 9, 10]);
    assert_eq!(slices.next(), None);
    let mut value = 8;
    for item in slicevec.iter_items() {
        assert_eq!(value, *item);
        value += 1
    }

    // Empty
    slicevec.remove_slice(c);
    assert_eq!(slicevec.items_len(), 0);
    assert_eq!(slicevec.slices_len(), 0);
}


#[test]
fn non_default_values() {
    struct Test; // No default implementation
    let mut slices = SliceMap::new(); // No default here either, new() instead.
    slices.add_items([Test, Test, Test]);
    slices.add_items([Test]);
    assert_eq!(slices.items_len(), 4);
    assert_eq!(slices.slices_len(), 2);
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
