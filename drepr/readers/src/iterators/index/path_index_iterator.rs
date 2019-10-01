use crate::ra_reader::RAReader;

/// An path
///
#[derive(Debug)]
pub struct PathIndexIterator<'a> {
  ra_reader: &'a Box<dyn RAReader>,
}