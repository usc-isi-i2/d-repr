use serde::Deserialize;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
pub struct InputDataNode {
  pub class_id: String,
  pub class_name: String,
  pub predicate: String,
  pub data_type: Option<String>,
}

lazy_static! {
  static ref REG_FULL_STYPE: Regex = Regex::new(r"^((.+):\d+)--(.+)\^\^(.+)$").unwrap();
  static ref REG_STYPE: Regex = Regex::new(r"^((.+):\d+)--(.+)$").unwrap();
  static ref REG_CLASS_ID: Regex = Regex::new(r"^((.+):\d+)$").unwrap();
}

impl<'de> Deserialize<'de> for InputDataNode {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct TmpType {
      class_id: String,
      predicate: String,
      data_type: Option<String>,
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrObject {
      Str(String),
      Obj(TmpType),
    }

    match StringOrObject::deserialize(deserializer)? {
      StringOrObject::Str(s) => {
        let re: &Regex = if s.find("^^").is_some() {
          &REG_FULL_STYPE
        } else {
          &REG_STYPE
        };
        let m = re.captures(&s).expect("shorthand for data node should follow the format: <class_name>:<id_no>--<predicate>(^^<data_type>)?");
        Ok(InputDataNode {
          class_id: m.get(1).unwrap().as_str().to_string(),
          class_name: m.get(2).unwrap().as_str().to_string(),
          predicate: m.get(3).unwrap().as_str().to_string(),
          data_type: match m.get(4) {
            None => None,
            Some(g3) => Some(g3.as_str().to_string()),
          },
        })
      }
      StringOrObject::Obj(o) => {
        let n = REG_CLASS_ID
          .captures(&o.class_id)
          .expect("class id should follow the format: <class_name>:<id_no>");

        Ok(InputDataNode {
          class_id: n.get(1).unwrap().as_str().to_string(),
          class_name: n.get(2).unwrap().as_str().to_string(),
          predicate: o.predicate,
          data_type: o.data_type,
        })
      }
    }
  }
}
