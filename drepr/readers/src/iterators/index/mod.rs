use crate::index::Index;

mod known_range_iterator;
mod path_index_iterator;
mod unknown_range_iterator;

pub use self::known_range_iterator::{KnownRangeIter, KnownRangeRefIter};
pub use self::unknown_range_iterator::{UnknownRangeIter, UnknownRangeRefIter};
use std::fmt::Debug;

pub trait IndexIterator: Debug {
  /// get current value of the iterator
  fn value(&self) -> &[Index];

  /// get current value (mutable) of the iterator
  fn mut_value(&mut self) -> &mut [Index];

  /// move to the next value, return false when there is no extra values
  fn advance(&mut self) -> bool;
  fn freeze_last_step(&mut self);
}
