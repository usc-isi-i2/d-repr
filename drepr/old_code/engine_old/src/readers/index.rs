#[derive(Debug, Clone, PartialEq)]
pub enum Index {
  Str(String),
  Idx(usize),
}

impl Index {
  #[inline]
  pub fn as_mut_idx(&mut self) -> &mut usize {
    return match self {
      Index::Idx(idx) => idx,
      _ => panic!("Cannot convert string index to number index"),
    };
  }

  #[inline]
  pub fn as_idx(&self) -> usize {
    return match self {
      Index::Idx(idx) => *idx,
      _ => panic!("Cannot convert string index to number index: {:?}", self),
    };
  }

  #[inline]
  pub fn as_str(&self) -> &str {
    match self {
      Index::Str(s) => s,
      _ => panic!("Cannot convert number index to string index"),
    }
  }

  #[inline]
  pub fn set_idx(&mut self, new_idx: usize) {
    match self {
      Index::Idx(idx) => {
        *idx = new_idx;
      },
      _ => panic!("Cannot convert string index to number index"),
    };
  }
}