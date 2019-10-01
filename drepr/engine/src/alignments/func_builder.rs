use readers::prelude::RAReader;
use crate::lang::{Alignment, RangeAlignment, Description, ValueAlignment, Cardinality};
use crate::alignments::AlignmentFunc;
use crate::alignments::funcs::sgl_range_align::SRangeAlignFunc;
use crate::alignments::funcs::identity_align::IdenticalAlignment;
use crate::alignments::funcs::mul_range_align::MRangeAlignFunc;
use crate::alignments::funcs::sgl_value_align::SglValueAlignFunc;
use crate::alignments::funcs::mul_value_align::MulValueAlignFunc;
use crate::alignments::funcs::mul_chain_align::{MulChainMIncAlign, MulChainMDupAlign};
use crate::alignments::funcs::sgl_chain_align::SglChainAlign;


pub fn build_align_func<'a>(readers: &'a [Box<dyn RAReader + 'a>], desc: &Description, aligns: &[Alignment]) -> AlignmentFunc<'a> {
  if aligns.len() == 1 {
    return match &aligns[0] {
      Alignment::RangeAlign(da) => {
        build_range_align_func(readers, desc, da)
      }
      Alignment::ValueAlign(va) => {
        build_value_align_func(readers, desc, va)
      }
      Alignment::IdenticalAlign => {
        AlignmentFunc::Single(Box::new(IdenticalAlignment {}))
      }
    };
  }

  let align_funcs = aligns.iter()
    .map(|align| {
      return match align {
        Alignment::RangeAlign(da) => {
          build_range_align_func(readers, desc, da)
        }
        Alignment::ValueAlign(va) => {
          build_value_align_func(readers, desc, va)
        }
        Alignment::IdenticalAlign => {
          AlignmentFunc::Single(Box::new(IdenticalAlignment {}))
        }
      };
    })
    .collect::<Vec<_>>();
  let mut attrs = aligns.iter()
    .filter(|align| !align.is_identical_align())
    .map(|align| &desc.attributes[align.get_target()])
    .collect::<Vec<_>>();
  attrs.pop();

  // if it is always one2one & one2many, there will be no duplication
  let no_duplication = aligns
    .iter()
    .all(|align| {
      match align.compute_cardinality(desc) {
        Cardinality::O2O => true,
        Cardinality::O2M => true,
        _ => false
      }
    });
  let is_one2one = aligns
    .iter()
    .all(|align| align.compute_cardinality(desc) == Cardinality::O2O);
  
  if is_one2one {
    AlignmentFunc::Single(Box::new(SglChainAlign::new(readers, attrs, align_funcs.into_iter().map(|f| f.into_single()).collect())))
  } else if no_duplication {
    AlignmentFunc::Multiple(Box::new(MulChainMIncAlign::new(readers, attrs, align_funcs)))
  } else {
    AlignmentFunc::Multiple(Box::new(MulChainMDupAlign::new(readers, attrs, align_funcs)))
  }
}

pub fn build_range_align_func<'a>(readers: &'a [Box<dyn RAReader + 'a>], desc: &Description, align: &RangeAlignment) -> AlignmentFunc<'a> {
  if desc.attributes[align.target].path.get_no_nary_steps() > align.aligned_dims.len() {
    // this is *-to-many
    AlignmentFunc::Multiple(Box::new(
      MRangeAlignFunc::from_dim_align(&readers[desc.attributes[align.target].resource_id], desc, align)
    ))
  } else {
    AlignmentFunc::Single(Box::new(
      SRangeAlignFunc::from_dim_align(&desc, &align)
    ))
  }
}

pub fn build_value_align_func<'a>(readers: &'a [Box<dyn RAReader + 'a>], desc: &Description, align: &ValueAlignment) -> AlignmentFunc<'a> {
  let target = &desc.attributes[align.target];
  if target.unique {
    AlignmentFunc::Single(Box::new(
      SglValueAlignFunc::new(&readers[target.resource_id], target)
    ))
  } else {
    AlignmentFunc::Multiple(Box::new(MulValueAlignFunc::new(&readers[target.resource_id], target)))
  }
}

//pub fn build_chain_align_func<'a>(readers: &'a [Box<dyn RAReader + 'a>], desc: &Description, align:) {
//  unimplemented!()
//}