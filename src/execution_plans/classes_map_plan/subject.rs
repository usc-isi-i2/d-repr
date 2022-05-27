use hashbrown::HashSet;
use serde::{Serialize, Serializer};
use serde::ser::SerializeSeq;
use readers::into_enum_type_impl;
use readers::prelude::Value;

use crate::lang::{Alignment, Attribute};

use super::super::pseudo_id::ClassPseudoID;

/// A plan of generating subjects of a class. It contains an attribute, whose values are
/// one-to-one or many-to-one (because of denormalization) to other attributes values (need no duplication).
#[derive(Serialize, Debug)]
pub enum Subject<'a> {
  BlankSubject(BlankSubject<'a>),
  InternalIDSubject(InternalIDSubject<'a>),
  ExternalIDSubject(ExternalIDSubject<'a>),
}

/// A plan for blank subjects
#[derive(Serialize, Debug)]
pub struct BlankSubject<'a> {
  pub attr: &'a Attribute,
  pub pseudo_id: ClassPseudoID,
}

/// A plan for subject that is an attribute of the class
#[derive(Serialize, Debug)]
pub struct InternalIDSubject<'a> {
  pub attr: &'a Attribute,
  pub pseudo_id: ClassPseudoID,
  pub is_optional: bool,
  #[serde(serialize_with = "serialize_set")]
  pub missing_values: HashSet<Value>
}

/// A plan for subject that is an attribute outside of the class
#[derive(Serialize, Debug)]
pub struct ExternalIDSubject<'a> {
  pub attr: &'a Attribute,
  pub real_id: (&'a Attribute, Vec<Alignment>),
  pub pseudo_id: ClassPseudoID,
  pub is_optional: bool,
  #[serde(serialize_with = "serialize_set")]
  pub missing_values: HashSet<Value>
}

impl<'a> Subject<'a> {
  into_enum_type_impl!(Subject, into_blank_subject, BlankSubject, "BlankSubject", BlankSubject<'a>);
  into_enum_type_impl!(Subject, into_internal_id_subject, InternalIDSubject, "InternalIDSubject", InternalIDSubject<'a>);
  into_enum_type_impl!(Subject, into_external_id_subject, ExternalIDSubject, "ExternalIDSubject", ExternalIDSubject<'a>);

  pub fn get_attr(&self) -> &'a Attribute {
    match self {
      Subject::BlankSubject(subj) => subj.attr,
      Subject::InternalIDSubject(subj) => subj.attr,
      Subject::ExternalIDSubject(subj) => subj.attr,
    }
  }
  
  pub fn is_optional(&self) -> bool {
    match self {
      Subject::BlankSubject(_) => true,
      Subject::InternalIDSubject(s) => s.is_optional,
      Subject::ExternalIDSubject(s) => s.is_optional,
    }
  }
  
  /// Test if the subject attribute may contain duplicated values or not
  pub fn is_unique(&self) -> bool {
    match self {
      // always true for blank
      Subject::BlankSubject(_) => true,
      Subject::InternalIDSubject(s) => s.attr.unique,
      Subject::ExternalIDSubject(s) => s.real_id.0.unique,
    }
  }
}

pub fn serialize_set<S>(obj: &HashSet<Value>, s: S) -> Result<S::Ok, S::Error>
where S: Serializer {
  let mut map = s.serialize_seq(Some(obj.len()))?;
  for v in obj {
    map.serialize_element(v)?;
  }
  map.end()
}

