use crate::readers::*;

mod known_size_iterator;
mod known_size_ref_iterator;
mod unknown_size_iterator;
mod unknown_size_ref_iterator;
mod insert_iterator;

pub trait StreamingIndexIterator {
  #[inline]
  fn value(&self) -> &[Index];
  #[inline]
  fn mut_value(&mut self) -> &mut [Index];
  fn advance(&mut self) -> bool;
  fn freeze_last_index(&mut self);
}

pub use self::known_size_iterator::KnownSizeIter;
pub use self::known_size_ref_iterator::KnownSizeRefIter;
pub use self::unknown_size_iterator::UnknownSizeIter;
pub use self::unknown_size_ref_iterator::UnknownSizeRefIter;
pub use self::insert_iterator::InsertIterator;