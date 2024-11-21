use crate::SlotSliceMap;
use slotmap::new_key_type;

extern crate alloc;
use alloc::vec::Vec;

new_key_type! {
    pub struct TestKey;
}

// #[test]
// fn doc_test(){
// }

#[test]
fn test_basic() {
    let mut slicemap = SlotSliceMap::<TestKey, i32>::new();
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
    let mut slicemap = SlotSliceMap::<TestKey, i32>::new();

    let a = slicemap.add_items([1, 2, 3, 4, 5]);
    let b = slicemap.add_items([6, 7]);
    let c = slicemap.add_items([8, 9, 10]);
    assert_eq!(slicemap.items_len(), 10);
    assert_eq!(slicemap.slices_len(), 3);

    // Remove
    slicemap.remove_slice(b);

    // Iterating over slices
    assert_eq!(slicemap.slices_len(), 2);
    let mut slices = slicemap.iter_slices();
    assert_eq!(slices.next().unwrap(), [1, 2, 3, 4, 5]);
    assert_eq!(slices.next().unwrap(), [8, 9, 10]);
    assert_eq!(slices.next(), None);
    drop(slices);

    // Iterating over all items
    let mut value = 1;
    for (i, item) in slicemap.iter_items().enumerate() {
        if i < 5 {
            assert_eq!(value, *item);
        } else {
            assert_eq!(value + 2, *item);
        }
        value += 1
    }

    // Remove and test again
    slicemap.remove_slice(a);
    assert_eq!(slicemap.slices_len(), 1);
    let mut slices = slicemap.iter_slices();
    assert_eq!(slices.next().unwrap(), [8, 9, 10]);
    assert_eq!(slices.next(), None);
    let mut value = 8;
    for item in slicemap.iter_items() {
        assert_eq!(value, *item);
        value += 1
    }
    drop(slices);

    // Empty
    slicemap.remove_slice(c);
    assert_eq!(slicemap.items_len(), 0);
    assert_eq!(slicemap.slices_len(), 0);
}

#[test]
fn non_default_values() {
    struct Test; // No default implementation
    let mut slices = SlotSliceMap::<TestKey, Test>::new();
    slices.add_items([Test, Test, Test]);
    slices.add_items([Test]);
    assert_eq!(slices.items_len(), 4);
    assert_eq!(slices.slices_len(), 2);
}
