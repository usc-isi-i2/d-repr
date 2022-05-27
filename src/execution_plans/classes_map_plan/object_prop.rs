use hashbrown::HashSet;
use serde::{Serialize};

use readers::{into_enum_type_impl, as_enum_type_impl, is_enum_type_impl};
use readers::prelude::Value;

use crate::lang::{Alignment, Attribute, Cardinality};

use super::super::pseudo_id::ClassPseudoID;

#[derive(Serialize, Debug)]
pub enum ObjectProp<'a> {
  BlankObject(BlankObject<'a>),
  IDObject(IDObject<'a>),
}

#[derive(Serialize, Debug)]
pub struct BlankObject<'a> {
  pub attribute: &'a Attribute,
  pub alignments: Vec<Alignment>,
  pub alignments_cardinality: Cardinality,
  pub pseudo_id: ClassPseudoID,
  pub predicate_id: usize,
  pub class_id: usize,
  pub is_optional: bool,
  // if the target class is optional
  pub is_target_optional: bool,
}

#[derive(Serialize, Debug)]
pub struct IDObject<'a> {
  pub attribute: &'a Attribute,
  pub alignments: Vec<Alignment>,
  pub alignments_cardinality: Cardinality,
  pub pseudo_id: ClassPseudoID,
  pub predicate_id: usize,
  pub class_id: usize,
  pub is_optional: bool,
  pub is_target_optional: bool,
  #[serde(serialize_with = "super::subject::serialize_set")]
  pub missing_values: HashSet<Value>,
}

impl<'a> ObjectProp<'a> {
  is_enum_type_impl!(ObjectProp::is_blank_object(BlankObject(_)));
  is_enum_type_impl!(ObjectProp::is_id_object(IDObject(_)));
  as_enum_type_impl!(ObjectProp, as_blank_object, as_mut_blank_object, BlankObject, "BlankObject", BlankObject<'a>);
  as_enum_type_impl!(ObjectProp, as_id_object, as_mut_id_object, IDObject, "IDObject", IDObject<'a>);
  into_enum_type_impl!(ObjectProp, into_blank_object, BlankObject, "BlankObject", BlankObject<'a>);
  into_enum_type_impl!(ObjectProp, into_id_object, IDObject, "IDObject", IDObject<'a>);
  
  #[inline]
  pub fn get_attr(&self) -> &'a Attribute {
    match self {
      ObjectProp::BlankObject(obj) => obj.attribute,
      ObjectProp::IDObject(obj) => obj.attribute,
    }
  }
  
  #[inline]
  pub fn get_alignments(&self) -> &[Alignment] {
    match self {
      ObjectProp::BlankObject(obj) => &obj.alignments,
      ObjectProp::IDObject(obj) => &obj.alignments
    }
  }
  
  #[inline]
  pub fn is_optional(&self) -> bool {
    match self {
      ObjectProp::BlankObject(obj) => obj.is_optional,
      ObjectProp::IDObject(obj) => obj.is_optional
    }
  }
}