use readers::path_expr::PathExpr;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum PreprocessingFunc {
  #[serde(rename = "pmap")]
  PyMap(PyMap),
  #[serde(rename = "pfilter")]
  PyFilter(PyFilter),
  #[serde(rename = "psplit")]
  PySplit(PySplit),
  #[serde(rename = "rmap")]
  RuMap(RuMap),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PyMap {
  pub resource_id: usize,
  pub path: PathExpr,
  pub output: Option<usize>,
  pub change_structure: Option<bool>,
  pub code: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PyFilter {
  pub resource_id: usize,
  pub path: PathExpr,
  pub output: Option<usize>,
  pub code: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PySplit {
  pub resource_id: usize,
  pub path: PathExpr,
  pub output: Option<usize>,
  pub code: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RuMap {
  pub resource_id: usize,
  pub path: PathExpr,
  pub func_id: BuiltinRustMapFunc,
  pub output: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "t")]
pub enum BuiltinRustMapFunc {
  #[serde(rename = "dict2items")]
  Dict2Items
}