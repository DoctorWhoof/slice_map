### Update:
Removed "SliceArray" for now. Focusing on a solid SliceVec implementation, then will reinstate SliceArray and update it to match SliceVec's features.

SliceMap provides a container that allows iterating directly all of its items, or iterating through non-overlapping slices of varying sizes. You can only insert new items in groups that will become a new slice.

One implementation is provided out of the box, [SliceVec], but you can extend [SliceMap] to use your own storage by implementing the [Storage] trait.

### Examples

A good use would be storing the points for polygons of different point counts, but in a way where all those points are laid out continuously in memory. Each slice is effectively a new polygon, and drawing all polygons at once can be very CPU cache-friendly.

Here's the code for a simpler example with i32 values:
```rust
use slice_map::SliceVec;
let mut slicevec = SliceVec::default();

// Adding items can fail when using SliceArray and the capacity is reached,
// but is unlikely to fail with SliceVec.
slicevec.add_items([1, 2, 3, 4, 5]).ok();
slicevec.add_items([6, 7]).ok();
slicevec.add_items([8, 9, 10]).ok();

// Iterating over slices
let mut slices = slicevec.iter_slices();
assert_eq!(slices.next().unwrap(), [1, 2, 3, 4, 5]);
assert_eq!(slices.next().unwrap(), [6, 7]);
assert_eq!(slices.next().unwrap(), [8, 9, 10]);
assert_eq!(slices.next(), None);

// Iterating over all items
let mut i = 1;
for item in slicevec.iter_items(){
    assert_eq!(i, *item);
    i += 1
}
```

### Features:
#### "default"
The "vec" feature is enabled by default. If you want to use a no_std environment without Vecs, you can
disable default features and implement "[Storage]" for your desired container, which will allow you to create a SliceMap with it.

#### "vec"
Enables [SliceVec], which is a SliceMap implementation using Vecs for all its storage. This is the easiest to use option.
