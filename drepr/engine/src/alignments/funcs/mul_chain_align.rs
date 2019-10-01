use crate::alignments::{MAlignmentFunc, AlignmentFunc};
use readers::prelude::{Index, Value, IndexIterator};
use readers::ra_reader::RAReader;
use crate::alignments::funcs::iters::aligns_iter::AlignsIter;
use crate::execution_plans::pseudo_id::ClassPseudoID;
use crate::lang::Attribute;
use crate::alignments::funcs::iters::dedup_iter::DedupIndexIter;

/// Chain alignment, which requires all alignments to be one-to-one or one-to-many (no duplication)
///
/// The chain alignment is represented by a list of `funcs = [f_0, f_1, ..., f_n]`, in which
/// `f_0` is the alignment between source attribute x and y0, f_1 between y0 & y1, ..., f_n between
/// y_n-1 and y_n (y_n is also a z, our target attribute).
///
/// It has a list of intermediate vector of index of y_0 .. y_n-1 (because y_n or z is provided in the call)
/// It has a list of readers, each reader at position i contains data of y_i
/// The pseudo id is to generate id of the target attribute for filtering duplication.
///
/// In summary: at position i, ys[i] contains index vector of attribute y_i, the data of y_i is stored
/// in readers[i], the alignment function at i (funcs[i]) is between attribute y_i-1 and y_i.
#[derive(Debug)]
pub struct MulChainMIncAlign<'a> {
  readers: Vec<&'a Box<dyn RAReader + 'a>>,
  ys: Vec<Box<Vec<Index>>>,
  funcs: Vec<AlignmentFunc<'a>>,
}

/// Chain alignment, which handle duplications
///
/// The chain alignment is represented by a list of `funcs = [f_0, f_1, ..., f_n]`, in which
/// `f_0` is the alignment between source attribute x and y0, f_1 between y0 & y1, ..., f_n between
/// y_n-1 and y_n (y_n is also a z, our target attribute).
///
/// It has a list of intermediate vector of index of y_0 .. y_n-1 (because y_n or z is provided in the call)
/// It has a list of readers, each reader at position i contains data of y_i
/// The pseudo id is to generate id of the target attribute for filtering duplication.
///
/// In summary: at position i, ys[i] contains index vector of attribute y_i, the data of y_i is stored
/// in readers[i], the alignment function at i (funcs[i]) is between attribute y_i-1 and y_i.
#[derive(Debug)]
pub struct MulChainMDupAlign<'a> {
  readers: Vec<&'a Box<dyn RAReader + 'a>>,
  ys: Vec<Box<Vec<Index>>>,
  funcs: Vec<AlignmentFunc<'a>>,
  pseudo_id: ClassPseudoID
}

impl<'a> MulChainMIncAlign<'a> {
  pub fn new(readers: &'a [Box<dyn RAReader + 'a>], attrs: Vec<&Attribute>, funcs: Vec<AlignmentFunc<'a>>) -> MulChainMIncAlign<'a> {
    MulChainMIncAlign { 
      readers: attrs.iter().map(|a| &readers[a.resource_id]).collect::<Vec<_>>(),
      ys: attrs.iter()
        .map(|a| Box::new(a.path.get_initial_step(readers[a.resource_id].as_ref()))).collect::<Vec<_>>(),
      funcs 
    }
  }
}

impl<'a0> MAlignmentFunc for MulChainMIncAlign<'a0> {
  fn iter_alignments<'a1: 'a, 'a>(&'a1 mut self, source: &[Index], source_val: &Value, target: &'a mut [Index]) -> Box<dyn IndexIterator + 'a> {
    Box::new(AlignsIter::new(&self.readers, &mut self.funcs, &mut self.ys, source, source_val, target))
  }
}

impl<'a> MulChainMDupAlign<'a> {
  /// `attrs`: vector of attributes does not include the source
  ///
  pub fn new(readers: &'a [Box<dyn RAReader + 'a>], attrs: Vec<&Attribute>, funcs: Vec<AlignmentFunc<'a>>) -> MulChainMDupAlign<'a> {
    MulChainMDupAlign { 
      readers: attrs.iter().map(|a| &readers[a.resource_id]).collect::<Vec<_>>(),
      ys: attrs.iter()
        .map(|a| Box::new(a.path.get_initial_step(readers[a.resource_id].as_ref()))).collect::<Vec<_>>(),
      funcs,
      pseudo_id: ClassPseudoID::new("".to_string(), attrs.last().unwrap().path.get_nary_steps())
    }
  }
}

impl<'a0> MAlignmentFunc for MulChainMDupAlign<'a0> {
  fn iter_alignments<'a1: 'a, 'a>(&'a1 mut self, source: &[Index], source_val: &Value, target: &'a mut [Index]) -> Box<dyn IndexIterator + 'a> {
    Box::new(DedupIndexIter::new(
      AlignsIter::new(&self.readers, &mut self.funcs, &mut self.ys, source, source_val, target),
      &self.pseudo_id
    ))
  }
}
