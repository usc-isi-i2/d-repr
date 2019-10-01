use hashbrown::HashSet;
use readers::prelude::{IndexIterator, Index};
use crate::execution_plans::pseudo_id::ClassPseudoID;

#[derive(Debug)]
pub struct DedupIndexIter<'a, I: IndexIterator> {
  pub iter: I,
  pub pseudo_id: &'a ClassPseudoID,
  pub generated_indices: HashSet<String>
}

impl<'a, I: IndexIterator> DedupIndexIter<'a, I> {
  pub fn new(iter: I, pseudo_id: &'a ClassPseudoID) -> DedupIndexIter<'a, I> {
    DedupIndexIter {
      iter,
      pseudo_id,
      generated_indices: HashSet::new()
    }
  }
}

impl<'a, I: IndexIterator> IndexIterator for DedupIndexIter<'a, I> {
  fn value(&self) -> &[Index] {
    self.iter.value()
  }
  
  fn mut_value(&mut self) -> &mut [Index] {
    self.iter.mut_value()
  }
  
  fn advance(&mut self) -> bool {
    loop {
      if !self.iter.advance() {
        return false;
      }
      
      let pid = self.pseudo_id.get_id_string(self.iter.value());
      if !self.generated_indices.contains(&pid) {
        self.generated_indices.insert(pid);
        return true;
      }
    }
  }
  
  fn freeze_last_step(&mut self) {
    unreachable!()
  }
}