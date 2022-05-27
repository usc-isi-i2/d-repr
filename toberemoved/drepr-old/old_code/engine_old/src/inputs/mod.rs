use fnv::FnvHashMap;
use hashbrown::HashMap;
use serde::Deserialize;

use crate::models::*;
use crate::python::PyExecutor;

use self::input_alignment::InputAlignment;
use self::input_preprocessing::*;
use self::input_resource::*;
use self::input_semantic_model::InputSemanticModel;
pub use self::input_slice::*;
pub use self::input_variable::*;

mod input_alignment;
mod input_preprocessing;
mod input_resource;
mod input_semantic_model;
mod input_slice;
mod input_variable;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct InputRepresentation {
  pub resources: InputResources,
  pub variables: FnvHashMap<String, InputVariable>,
  #[serde(default)]
  pub alignments: Vec<InputAlignment>,
  #[serde(default)]
  pub preprocessing: Vec<PreprocessFuncFactory>,
  #[serde(default)]
  pub semantic_model: Option<InputSemanticModel>,
}

#[derive(Default, Debug)]
pub struct InputPhysicalResource {
  pub resource_id: String,
  pub resource_file: String,
}

impl InputRepresentation {
  pub fn get_resources(&self) -> &FnvHashMap<String, Resource> {
    &self.resources.resources
  }

  pub fn should_have_multiple_readers(&self) -> bool {
    for f in &self.preprocessing {
      let m_readers = match f {
        PreprocessFuncFactory::PMap(v) => v.output.is_some(),
        PreprocessFuncFactory::PFilter(v) => v.output.is_some(),
        PreprocessFuncFactory::RMap(v) => v.output.is_some()
      };

      if m_readers {
        return true;
      }
    }

    return false;
  }

  pub fn into_repr(mut self, py_executor: &mut PyExecutor) -> Representation {
    let mut has_multiple_resources = true;
    if self.resources.resources.len() == 1 {
      // if there is only one resource and no new resource created by preprocessing
      // we have to remove resource id
      if self.should_have_multiple_readers() {
        // new resource, so we have to set resource id to the current source
        let rid = self.resources.resources.keys().next().unwrap();
        for var in self.variables.values_mut() {
          if var.location.resource_id.is_none() {
            var.location.resource_id = Some(rid.clone());
          }
        }

        for func in self.preprocessing.iter_mut() {
          match func {
            PreprocessFuncFactory::PMap(pfunc) => {
              if pfunc.input.resource_id.is_none() {
                pfunc.input.resource_id = Some(rid.clone());
              }
            }
            PreprocessFuncFactory::PFilter(pfunc) => {
              if pfunc.input.resource_id.is_none() {
                pfunc.input.resource_id = Some(rid.clone());
              }
            }
            PreprocessFuncFactory::RMap(rfunc) => {
              if rfunc.input.resource_id.is_none() {
                rfunc.input.resource_id = Some(rid.clone());
              }
            }
          }
        }
      } else {
        // no resource, we have to remove resource id
        for var in self.variables.values_mut() {
          var.location.resource_id = None;
        }

        for func in self.preprocessing.iter_mut() {
          match func {
            PreprocessFuncFactory::PMap(pfunc) => {
              pfunc.input.resource_id = None;
            }
            PreprocessFuncFactory::PFilter(pfunc) => {
              pfunc.input.resource_id = None;
            }
            PreprocessFuncFactory::RMap(pfunc) => {
              pfunc.input.resource_id = None;
            }
          }
        }

        has_multiple_resources = false;
      }
    }

    let variables = self
      .variables
      .into_iter()
      .map(|(k, v)| Variable {
        name: k,
        location: v.location.into_location(py_executor),
        unique: v.unique,
        sorted: v.sorted,
        value_type: v.value_type,
      })
      .collect::<Vec<_>>();

    let preprocess_funcs = self
      .preprocessing
      .into_iter()
      .map(|f| match f {
        PreprocessFuncFactory::PMap(r) => PreprocessFunc::PMap(PMap {
          input: r.input.into_location(py_executor),
          output: r.output,
          code: py_executor
            .compile(&r.code)
            .expect("Invalid python function"),
        }),
        PreprocessFuncFactory::PFilter(r) => PreprocessFunc::PFilter(PFilter {
          input: r.input.into_location(py_executor),
          output: r.output,
          code: py_executor
            .compile(&r.code)
            .expect("Invalid python function"),
        }),
        PreprocessFuncFactory::RMap(r) => PreprocessFunc::RMap(RMap {
          func_id: "rmap-dict2items".to_string(),
          input: r.input.into_location(py_executor),
          output: r.output
        })
      })
      .collect::<Vec<_>>();

    let semantic_model;
    let alignments;
    {
      let var_name2id: HashMap<&str, usize> = variables
        .iter()
        .enumerate()
        .map(|(k, v)| (v.name.as_str(), k))
        .collect();

      semantic_model = match self.semantic_model {
        None => SemanticModel::from_variables(&variables),
        Some(sm) => {
          if sm.is_empty() {
            SemanticModel::from_variables(&variables)
          } else {
            sm.into_semantic_model(&var_name2id)
          }
        }
      };
      alignments = self
        .alignments
        .into_iter()
        .map(|a| a.into_alignment_factory(&var_name2id, has_multiple_resources))
        .collect();
    }

    Representation {
      resources: self.resources.resources.clone(),
      preprocess_funcs,
      variables,
      alignments,
      semantic_model,
    }
  }
}
