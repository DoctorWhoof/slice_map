### 0.1.8 Update
Dramatically simplified from a generic SliceMap that supported different storages to a Concrete SliceMap that always uses Vec<V> for values and SlotMap<SliceKey, Range<u32>> for slice ranges. This now allows removing slices and the existing Slice Keys remain valid (except for the removed one), thanks to the SlotMap.

[SliceMap] provides a container that allows iterating directly all of its items, or iterating through non-overlapping slices of varying sizes. You can only insert new items in groups that will become a new slice.

### Example

A good use would be storing the points for polygons of different point counts, but in a way where all those points are laid out continuously in memory. Each slice is effectively a new polygon, and drawing all polygons at once can be very CPU cache-friendly.

Here's the code for a simpler example with i32 values:
```rust
use slice_map::SliceMap;
let mut slices = SliceMap::default();

// Adding items can fail when using SliceArray and the capacity is reached,
// but is unlikely to fail with SliceVec.
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
```
