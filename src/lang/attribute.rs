use serde::{Deserialize, Serialize};
use readers::prelude::PathExpr;
use readers::value::Value;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Attribute {
  pub id: usize,
  pub resource_id: usize,
  pub path: PathExpr,
  pub unique: bool,
  pub sorted: SortedOption,
  pub vtype: ValueType,
  pub missing_values: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum SortedOption {
  #[serde(rename = "none")]
  Null,
  #[serde(rename = "ascending")]
  Ascending,
  #[serde(rename = "descending")]
  Descending
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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