use super::location::Location;
use super::location::Slice;
use serde::{Serialize, Deserialize};
use crate::readers::Index;

#[derive(Debug, Clone)]
pub struct Variable {
  pub name: String,
  pub location: Location,
  pub unique: bool,
  pub sorted: VariableSorted,
  pub value_type: ValueType
}

impl Variable {
  /// return the first index of the first value of the variable in the data source
  pub fn get_first_index(&self) -> Vec<Index> {
    let mut index = vec![];
    for s in &self.location.slices {
      match s {
        Slice::Range(r) => {
          index.push(Index::Idx(r.start));
        },
        Slice::Index(i) => {
          index.push(i.idx.clone());
        }
      }
    }

    index
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariableSorted {
  #[serde(rename = "none")]
  Null,
  #[serde(rename = "ascending")]
  Ascending,
  #[serde(rename = "descending")]
  Descending
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
  #[serde(rename = "unspecified")]
  Unspecified,
  #[serde(rename = "int")]
  Integer,
  #[serde(rename = "float")]
  Float,
  #[serde(rename = "str")]
  Str,
  #[serde(rename = "list[int]")]
  IntArray,
  #[serde(rename = "list[float]")]
  FloatArray,
  #[serde(rename = "list[str]")]
  StrArray
}
