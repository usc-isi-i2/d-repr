use crate::index::Index;

mod known_range_iterator;
mod unknown_range_iterator;
//mod path_index_iterator;

pub use self::known_range_iterator::{KnownRangeIter, KnownRangeRefIter};
pub use self::unknown_range_iterator::{UnknownRangeIter, UnknownRangeRefIter};
use std::fmt::Debug;

pub trait IndexIterator: Debug {
  #[inline]
  fn value(&self) -> &[Index];

  #[inline]
  fn mut_value(&mut self) -> &mut [Index];

  fn advance(&mut self) -> bool;

  fn freeze_last_step(&mut self);
}
