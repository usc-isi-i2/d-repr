use crate::readers::Index;

#[derive(Debug, Clone)]
pub enum Slice {
  Range(RangeSlice),
  Index(IndexSlice),
}

impl Slice {
  pub fn range(start: usize, end: Option<i64>, step: usize) -> Slice {
    Slice::Range(RangeSlice { start, end, step })
  }

  pub fn index(idx: Index) -> Slice {
    Slice::Index(IndexSlice { idx })
  }

  pub fn is_range(&self) -> bool {
    match self {
      Slice::Range(_) => true,
      Slice::Index(_) => false,
    }
  }

  pub fn into_range(self) -> RangeSlice {
    match self {
      Slice::Range(r) => r,
      _ => panic!("Cannot convert non-range slice into range slice"),
    }
  }

  pub fn as_range(&self) -> &RangeSlice {
    match self {
      Slice::Range(r) => r,
      _ => panic!("Cannot convert non-range slice into range slice"),
    }
  }

  pub fn as_mut_range(&mut self) -> &mut RangeSlice {
    match self {
      Slice::Range(r) => r,
      _ => panic!("Cannot convert non-range slice into range slice"),
    }
  }

  pub fn as_index(&self) -> &IndexSlice {
    match self {
      Slice::Index(i) => i,
      _ => panic!("Cannot convert non-index slice into index slice"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct RangeSlice {
  pub start: usize,
  pub end: Option<i64>,
  pub step: usize,
}

impl RangeSlice {
  pub fn get_end(&self, n_elements: usize) -> usize {
    match self.end {
      None => n_elements,
      Some(v) => {
        if v < 0 {
          v as usize + n_elements
        } else {
          v as usize
        }
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct IndexSlice {
  pub idx: Index,
}

#[derive(Debug, Clone)]
pub struct Location {
  pub slices: Vec<Slice>,
}

impl Location {
  pub fn ignore_last_slice(&self) -> Location {
    Location {
      slices: self.slices[..self.slices.len() - 1]
        .iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>(),
    }
  }

  /// return a list of unbounded dimensions
  pub fn get_unbounded_dims(&self) -> Vec<usize> {
    let mut unbounded_dims = vec![];
    for (d, s) in self.slices.iter().enumerate() {
      if let Slice::Range(_) = s {
        unbounded_dims.push(d);
      }
    }

    unbounded_dims
  }

  pub fn get_first_index(&self) -> Vec<Index> {
    let mut idx = vec![];
    for s in &self.slices {
      match s {
        Slice::Index(i) => {
          idx.push(i.idx.clone());
        }
        Slice::Range(r) => {
          idx.push(Index::Idx(r.start));
        }
      }
    }
    idx
  }
}
