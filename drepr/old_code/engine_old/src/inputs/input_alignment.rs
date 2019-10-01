use serde::Deserialize;
use crate::models::{BasedAlignedDim, AlignmentFactory, DimAlignFactory, ValueAlignFactory};
use hashbrown::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum InputAlignment {
  #[serde(rename = "dimension")]
  Dimension(InputDimAlign),
  #[serde(rename = "value")]
  Value(InputValueAlign),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InputValueAlign {
  source: String,
  target: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputDimAlign {
  source: String,
  target: String,
  aligned_dims: Vec<BasedAlignedDim>,
}

lazy_static! {
  static ref REG_DALIGN: Regex = Regex::new(r"^(.+):(\d+) *<-> *(.+):(\d+)$").unwrap();
}

impl InputAlignment {
  pub fn into_alignment_factory(self, var_name2id: &HashMap<&str, usize>, has_multiple_resources: bool) -> AlignmentFactory {
    match self {
      InputAlignment::Dimension(x) => AlignmentFactory::DimAlign(DimAlignFactory {
        source: var_name2id[x.source.as_str()],
        target: var_name2id[x.target.as_str()],
        aligned_dims: if has_multiple_resources {
          x.aligned_dims.into_iter()
            .map(|adim| BasedAlignedDim {
              source_dim: adim.source_dim + 1,
              target_dim: adim.target_dim + 1
            })
            .collect::<Vec<_>>()
        } else {
          x.aligned_dims
        },
      }),
      InputAlignment::Value(x) => AlignmentFactory::ValueAlign(ValueAlignFactory {
        source: var_name2id[x.source.as_str()],
        target: var_name2id[x.target.as_str()],
      })
    }
  }
}

impl<'de> Deserialize<'de> for InputDimAlign {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TmpStruct {
      Full(Full),
      Short(Short),
    }

    #[derive(Deserialize)]
    struct Full {
      source: String,
      target: String,
      #[serde(default)]
      aligned_dims: Vec<BasedAlignedDim>,
    }

    #[derive(Deserialize)]
    struct Short {
      value: String
    }


    let res = match TmpStruct::deserialize(deserializer)? {
      TmpStruct::Full(f) => InputDimAlign {
        source: f.source,
        target: f.target,
        aligned_dims: f.aligned_dims,
      },
      TmpStruct::Short(s) => {
        let m = REG_DALIGN.captures(&s.value).unwrap();

        InputDimAlign {
          source: m.get(1).unwrap().as_str().to_string(),
          target: m.get(3).unwrap().as_str().to_string(),
          aligned_dims: vec![
            BasedAlignedDim {
              source_dim: m.get(2).unwrap().as_str().parse::<usize>().unwrap(),
              target_dim: m.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            }],
        }
      }
    };

    Ok(res)
  }
}