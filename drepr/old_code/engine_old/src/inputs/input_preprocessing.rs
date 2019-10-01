use serde::Deserialize;
use super::InputLocation;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum PreprocessFuncFactory {
  #[serde(rename = "pmap")]
  PMap(PMapFactory),
  #[serde(rename = "pfilter")]
  PFilter(PFilterFactory),
  #[serde(rename = "rmap-dict2items")]
  RMap(RMapFactory)
}

#[derive(Debug, Clone, Deserialize)]
pub struct PMapFactory {
  pub input: InputLocation,
  pub output: Option<String>,
  pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PFilterFactory {
  pub input: InputLocation,
  pub output: Option<String>,
  pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RMapFactory {
  pub input: InputLocation,
  pub output: Option<String>,
}