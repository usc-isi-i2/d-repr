use crate::models::*;
use crate::python::PyExecutor;
use crate::readers::Index;
use regex::Regex;
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
pub struct InputLocation {
  pub slices: Vec<InputSlice>,
  pub resource_id: Option<String>
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputSlice {
  Range(InputRangeSlice),
  Index(InputIndexSlice),
  RangeExpr(InputRangeExprSlice),
  IndexExpr(InputIndexExprSlice),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputRangeSlice {
  start: usize,
  end: Option<i64>,
  step: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputIndexSlice {
  idx: Index,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputIndexExprSlice {
  idx: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
enum StringOrUsize {
  Str(String),
  Usize(usize),
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
enum StringOrI64 {
  Str(String),
  I64(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputRangeExprSlice {
  start: StringOrUsize,
  end: Option<StringOrI64>,
  step: StringOrUsize,
}

impl InputLocation {
  pub fn into_location(self, py: &mut PyExecutor) -> Location {
    let mut slices = Vec::with_capacity(self.slices.len() + 1);
    if let Some(rid) = self.resource_id {
      slices.push(Slice::Index(IndexSlice {
        idx: Index::Str(rid)
      }));
    }

    for s in self.slices.into_iter() {
      slices.push(s.into_slice(py));
    }

    Location { slices }
  }
}

impl InputSlice {
  pub fn into_slice(self, py: &mut PyExecutor) -> Slice {
    match self {
      InputSlice::Range(s) => Slice::Range(RangeSlice {
        start: s.start,
        end: s.end,
        step: s.step,
      }),
      InputSlice::Index(s) => Slice::Index(IndexSlice { idx: s.idx }),
      InputSlice::IndexExpr(ie) => {
        let idx = match py.eval::<Index>(&ie.idx) {
          Err(e) => panic!(e),
          Ok(idx) => idx,
        };

        Slice::Index(IndexSlice { idx })
      }
      InputSlice::RangeExpr(s) => {
        let start = match s.start {
          StringOrUsize::Str(code) => py.eval::<usize>(&code).unwrap(),
          StringOrUsize::Usize(v) => v,
        };

        let end = match s.end {
          None => None,
          Some(StringOrI64::Str(code)) => Some(py.eval::<i64>(&code).unwrap()),
          Some(StringOrI64::I64(v)) => Some(v),
        };

        let step = match s.step {
          StringOrUsize::Str(code) => py.eval::<usize>(&code).unwrap(),
          StringOrUsize::Usize(v) => v,
        };

        Slice::Range(RangeSlice { start, end, step })
      }
    }
  }
}

lazy_static! {
  static ref REG_RANGE: Regex = Regex::new(r"^(\d+)?\.\.(-?\d+)?(?::(\d+))?$").unwrap();
  static ref REG_INDEX: Regex = Regex::new(r"^(?:\$\{([^}]+)})|(\d+)|(.*)$").unwrap();
  static ref REG_EXPR_RANGE: Regex = Regex::new(r"^(?:(\d+)|(?:\$\{([^}]+)}))?\.\.(?:(-\d+)|(?:\$\{([^}]+)}))?(?::(\d+)|(?:\$\{([^}]+)}))?$").unwrap();
}

impl<'de> serde::Deserialize<'de> for InputLocation {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct TmpInputLoc {
      resource_id: String,
      slices: Vec<InputSlice>
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum InputLocEnum {
      NoResource(Vec<InputSlice>),
      WithResource(TmpInputLoc)
    }

    let res = match InputLocEnum::deserialize(deserializer)? {
      InputLocEnum::NoResource(slices) => {
        InputLocation {
          resource_id: None,
          slices
        }
      },
      InputLocEnum::WithResource(loc) => {
        InputLocation {
          resource_id: Some(loc.resource_id),
          slices: loc.slices
        }
      }
    };
    Ok(res)
  }
}

impl<'de> serde::Deserialize<'de> for InputSlice {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let input = StringOrUsize::deserialize(deserializer)?;
    match input {
      StringOrUsize::Str(s) => {
        match REG_RANGE.captures(&s) {
          None => {
            // if this doesn't match, we need to check if this contains expression or not
            match REG_EXPR_RANGE.captures(&s) {
              None => {
                let caps = REG_INDEX
                  .captures(&s)
                  .expect("TODO: fill in a meaningful error message");
                match caps.get(1) {
                  Some(c) => Ok(InputSlice::IndexExpr(InputIndexExprSlice {
                    idx: c.as_str().to_string(),
                  })),
                  None => match caps.get(2) {
                    Some(c) => Ok(InputSlice::Index(InputIndexSlice {
                      idx: Index::Idx(c.as_str().parse::<usize>().unwrap()),
                    })),
                    None => Ok(InputSlice::Index(InputIndexSlice {
                      idx: Index::Str(caps.get(3).unwrap().as_str().to_string()),
                    })),
                  },
                }
              }
              Some(caps) => {
                // use Index as String / Index
                let start = match caps.get(1) {
                  None => {
                    match caps.get(2) {
                      None => StringOrUsize::Usize(0), // default of start
                      Some(g) => StringOrUsize::Str(g.as_str().to_string()),
                    }
                  }
                  Some(g) => StringOrUsize::Usize(g.as_str().parse::<usize>().unwrap()),
                };
                let end = match caps.get(3) {
                  None => {
                    match caps.get(4) {
                      None => None, // default of end
                      Some(g) => Some(StringOrI64::Str(g.as_str().to_string())),
                    }
                  }
                  Some(g) => Some(StringOrI64::I64(g.as_str().parse::<i64>().unwrap())),
                };
                let step = match caps.get(5) {
                  None => {
                    match caps.get(6) {
                      None => StringOrUsize::Usize(1), // default of step
                      Some(g) => StringOrUsize::Str(g.as_str().to_string()),
                    }
                  }
                  Some(g) => StringOrUsize::Usize(g.as_str().parse::<usize>().unwrap()),
                };

                Ok(InputSlice::RangeExpr(InputRangeExprSlice {
                  start,
                  end,
                  step
                }))
              }
            }
          }
          Some(caps) => Result::Ok(InputSlice::Range(InputRangeSlice {
            start: match caps.get(1) {
              Some(x) => x
                .as_str()
                .parse::<usize>()
                .expect("start index must be number"),
              None => 0,
            },
            end: match caps.get(2) {
              Some(x) => Some(
                x.as_str()
                  .parse::<i64>()
                  .expect("end index must be number or none"),
              ),
              None => None,
            },
            step: match caps.get(3) {
              Some(x) => x
                .as_str()
                .parse::<usize>()
                .expect("step must be number or none"),
              None => 1,
            },
          })),
        }
      }
      StringOrUsize::Usize(v) => {
        Result::Ok(InputSlice::Index(InputIndexSlice { idx: Index::Idx(v) }))
      }
    }
  }
}
