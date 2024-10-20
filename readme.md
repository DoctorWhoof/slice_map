### 0.1.8 Update
Dramatically simplified from a generic SliceMap that supported different storages to a more concrete SliceMap that always uses Vec<V> for values and SlotMap<SliceKey, Range<u32>> for slice ranges. This now allows removing slices and the existing Slice Keys remain valid (except for the removed one), thanks to the SlotMap.

### Description

[SliceMap] provides a container that allows iterating directly all of its items, or iterating through non-overlapping slices of varying sizes. You can only insert new items in groups that will become a new slice.

### Example

A good use would be storing the points for polygons with different point counts, but in a way where all those points are laid out continuously in memory. Each slice of points can be iterated separately and is effectively a new polygon. Drawing all polygons at once can be very CPU cache-friendly.

Here's a simpler example with i32 values:
```rust
use slice_map::SliceMap;
let mut slices = SliceMap::default();

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

// Iterating over all items
let mut i = 1;
for item in slices.iter_items(){
    assert_eq!(i, *item);
    i += 1
}

// Removing slices removes all of their items,
// but other keys are still valid!
slices.remove_slice(b);
let slice_c = slices.get_slice(c).unwrap();
assert_eq!(slice_c, &[8, 9, 10]);

let mut slice_iter = slices.iter_slices();
assert_eq!(slice_iter.next().unwrap(), [1, 2, 3, 4, 5]);
assert_eq!(slice_iter.next().unwrap(), [8, 9, 10]);
```
