use serde::Deserialize;
use super::variable::Variable;
use super::semantic_model::SemanticModel;
use super::alignments::*;
use super::preprocess::PreprocessFunc;
use fnv::FnvHashMap;


#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum Resource {
  #[serde(rename = "csv")]
  CSV(CSVResource),
  #[serde(rename = "json")]
  JSON,
  #[serde(rename = "spreadsheet")]
  Spreadsheet,
  #[serde(rename = "netcdf")]
  NetCDF,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CSVResource {
  #[serde(default = "CSVResource::default_delimiter")]
  pub delimiter: String
}

impl CSVResource {
  pub fn get_delimiter(&self) -> u8 {
    if self.delimiter.as_bytes().len() > 1 {
        panic!("Delimiter must be one byte character");
      }

    self.delimiter.as_bytes()[0]
  }

  pub fn default_delimiter() -> String {
    return String::from(",");
  }
}

#[derive(Default, Debug)]
pub struct Representation {
  pub resources: FnvHashMap<String, Resource>,
  pub preprocess_funcs: Vec<PreprocessFunc>,
  pub variables: Vec<Variable>,
  pub alignments: Vec<AlignmentFactory>,
  pub semantic_model: SemanticModel
}

