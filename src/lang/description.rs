use super::preprocessing::PreprocessingFunc;
use super::attribute::Attribute;
use super::semantic_model::SemanticModel;
use super::alignment::Alignment;
use super::resource::Resource;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Description {
  pub resources: Vec<Resource>,
  pub preprocessing: Vec<PreprocessingFunc>,
  pub attributes: Vec<Attribute>,
  pub alignments: Vec<Alignment>,
  pub semantic_model: SemanticModel
}