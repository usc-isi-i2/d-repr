use serde::{Deserialize, Serialize};

pub use self::range_alignment::{AlignedDim, RangeAlignment};
pub use self::value_alignment::ValueAlignment;
use crate::lang::description::Description;
use readers::{is_enum_type_impl, as_enum_type_impl, into_enum_type_impl};

pub mod range_alignment;
pub mod value_alignment;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum Alignment {
  #[serde(rename = "range")]
  RangeAlign(RangeAlignment),
  #[serde(rename = "value")]
  ValueAlign(ValueAlignment),
  #[serde(rename = "identical")]
  IdenticalAlign
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Serialize)]
pub enum Cardinality {
  O2O,
  O2M,
  M2O,
  M2M,
}

impl Alignment {
  is_enum_type_impl!(Alignment::is_range_align(RangeAlign(_)));
  is_enum_type_impl!(Alignment::is_value_align(ValueAlign(_)));
  is_enum_type_impl!(Alignment::is_identical_align(IdenticalAlign));
  as_enum_type_impl!(Alignment, as_range_align, as_mut_range_align, RangeAlign, "RangeAlignment", RangeAlignment);
  as_enum_type_impl!(Alignment, as_value_align, as_mut_value_align, ValueAlign, "ValueAlignment", ValueAlignment);

  into_enum_type_impl!(Alignment, into_range_align, RangeAlign, "RangeAlignment", RangeAlignment);
  into_enum_type_impl!(Alignment, into_value_align, ValueAlign, "ValueAlignment", ValueAlignment);
  
  pub fn get_target(&self) -> usize {
    match self {
      Alignment::RangeAlign(ra) => ra.target,
      Alignment::ValueAlign(va) => va.target,
      Alignment::IdenticalAlign => unreachable!()
    }
  }
  
  pub fn swap(&self) -> Option<Alignment> {
    match self {
      Alignment::RangeAlign(x) => Some(Alignment::RangeAlign(x.swap())),
      Alignment::ValueAlign(x) => Some(Alignment::ValueAlign(x.swap())),
      Alignment::IdenticalAlign => Some(Alignment::IdenticalAlign),
    }
  }
  
  pub fn is_swappable(&self) -> bool {
    match self {
      Alignment::RangeAlign(_) => true,
      Alignment::ValueAlign(_) => true,
      Alignment::IdenticalAlign => true
    }
  }
  
  /// Compute the cardinality of an alignment
  ///
  /// The cardinality between attribute `x` and attribute `y` are defined as follows:
  ///
  /// 1. one-to-one: one item of `x` can only link to one item of `y` and vice versa.
  /// 2. one-to-many: one item of `x` can link to multiple items of `y`, but one item of `y` can only
  ///    link to one item of `x`.
  /// 3. many-to-one: the reversed case of one-to-many
  /// 4. many-to-many: multiple items of `x` can link to multiple items of `y` and vice versa.
  pub fn compute_cardinality(&self, desc: &Description) -> Cardinality {
    match self {
      Alignment::RangeAlign(da) => da.compute_cardinality(desc),
      Alignment::ValueAlign(va) => va.compute_cardinality(desc),
      Alignment::IdenticalAlign => Cardinality::O2O,
    }
  }
}

impl Cardinality {
  pub fn is_any2one(&self) -> bool {
    *self == Cardinality::O2O || *self == Cardinality::M2O
  }
}