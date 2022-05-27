use crate::models::*;
use serde::Deserialize;

use super::input_slice::*;

#[derive(Debug, Clone, PartialEq)]
pub struct InputVariable {
  pub location: InputLocation,
  pub unique: bool,
  pub sorted: VariableSorted,
  pub value_type: ValueType,
}

impl InputVariable {
  pub fn default_unique() -> bool {
    false
  }
  pub fn default_sorted() -> VariableSorted {
    VariableSorted::Null
  }
  pub fn default_value_type() -> ValueType {
    ValueType::Unspecified
  }
}

impl<'de> Deserialize<'de> for InputVariable {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct TmpVar {
      pub location: InputLocation,
      #[serde(default = "InputVariable::default_unique")]
      pub unique: bool,
      #[serde(default = "InputVariable::default_sorted")]
      pub sorted: VariableSorted,
      #[serde(default = "InputVariable::default_value_type")]
      pub value_type: ValueType,
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum LocationOrVariable {
      Loc(InputLocation),
      Var(TmpVar),
    }

    match LocationOrVariable::deserialize(deserializer)? {
      LocationOrVariable::Loc(loc) => Ok(InputVariable {
        location: loc,
        unique: false,
        sorted: VariableSorted::Null,
        value_type: ValueType::Unspecified,
      }),
      LocationOrVariable::Var(var) => Ok(InputVariable {
        location: var.location,
        unique: var.unique,
        sorted: var.sorted,
        value_type: var.value_type,
      }),
    }
  }
}
