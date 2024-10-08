#![no_std]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]

// Main.
mod array_vec;
pub use array_vec::*;

mod slice_map;
pub use slice_map::*;

mod storage;
pub use storage::*;

pub type StrResult = Result<(), &'static str>;
pub type Slice = core::ops::Range<u32>;

// Optional features.

#[cfg(feature = "vec")]
mod slice_vec;
#[cfg(feature = "vec")]
pub use slice_vec::*;

// #[cfg(feature = "array")]
mod slice_array;
// #[cfg(feature = "array")]
pub use slice_array::*;

// Tests.

#[cfg(test)]
pub(crate) mod test;
