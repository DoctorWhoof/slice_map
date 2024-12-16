### 0.2.4 Update

SliceMap is a Rust crate driven by needs of a separate personal project. As a result, I had to go back to the idea of a Generic SliceMap that uses a Storage trait to pick different Storage structs.

Instead of [SliceMap] you should use the new type aliases, [SlotSliceMap] for SlotMap storage, [SecSliceMap] for SecondaryMap and [SparseSliceMap] for SparseSecondaryMap respectively.

To allow using [SparseSecondaryMap] this crate is not "no_std" anymore, but I plan to make that an optional feature and restore its no_std status!

### Description

[SliceMap] and its type aliases provides a container that allows iterating directly all of its items, or iterating through non-overlapping slices of varying sizes. You can only insert new items in groups that will become a new slice.

### Example

A good use would be storing the points for polygons with different point counts, but in a way where all those points are laid out continuously in memory. Each slice of points can be iterated separately and is effectively a new polygon. Drawing all polygons at once can be very CPU cache-friendly.

Here's a simpler example with i32 values:

```rust
use slice_map::SlotSliceMap;

// Since SlotSLiceMap uses slotmap keys, we need to define our key in advance
slotmap::new_key_type!{
    pub struct TestKey;
}

let mut slices = SlotSliceMap::<TestKey, i32>::default();

// Adding items returns a SliceKey
let a = slices.add_items([1, 2, 3, 4, 5]);
let b = slices.add_items([6, 7]);
let c = slices.add_items([8, 9, 10]);

// Iterating over slices
let mut slice_iter = slices.iter_slices();
assert_eq!(slice_iter.next().unwrap(), [1, 2, 3, 4, 5]);
assert_eq!(slice_iter.next().unwrap(), [6, 7]);
assert_eq!(slice_iter.next().unwrap(), [8, 9, 10]);
assert_eq!(slice_iter.next(), None);
drop(slice_iter);

// Iterating over all items
let mut i = 1;
for item in slices.iter_items(){
    assert_eq!(i, *item);
    i += 1
}

 // Removing slices removes all of their items,
 // but other keys are still valid!
{
    slices.remove_slice(b);
    let slice_c = slices.get_slice(c).unwrap();
    assert_eq!(slice_c, &[8, 9, 10]);
}

// Iterating
{
    let mut slice_iter = slices.iter_slices();
    assert_eq!(slice_iter.next().unwrap(), [1, 2, 3, 4, 5]);
    assert_eq!(slice_iter.next().unwrap(), [8, 9, 10]);
}

slices.remove_slice(a);
slices.remove_slice(c);
assert!(slices.is_empty());
```
