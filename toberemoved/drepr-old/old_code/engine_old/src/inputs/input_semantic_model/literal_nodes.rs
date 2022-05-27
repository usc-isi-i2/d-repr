use regex::Regex;
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
pub struct InputLiteralNode {
  pub class_id: String,
  pub class_name: String,
  pub predicate: String,
  pub data: String,
  pub data_type: Option<String>,
}

lazy_static! {
  static ref REG_FULL_LITERAL: Regex = Regex::new(r"^((.+):\d+)--(.+)--(.+)\^\^(.+)?$").unwrap();
  static ref REG_LITERAL: Regex = Regex::new(r"^((.+):\d+)--(.+)--(.+)$").unwrap();
  static ref REG_CLASS_ID: Regex = Regex::new(r"^((.+):\d+)$").unwrap();
}

impl<'de> Deserialize<'de> for InputLiteralNode {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct TmpType {
      class_id: String,
      predicate: String,
      data: String,
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
          &REG_FULL_LITERAL
        } else {
          &REG_LITERAL
        };
        let m = re.captures(&s).expect("shorthand for literal node should follow the format: <class_name>:<id_no>--<prediate>--<value>(^^<data_type>)?");

        Ok(InputLiteralNode {
          class_id: m.get(1).unwrap().as_str().to_string(),
          class_name: m.get(2).unwrap().as_str().to_string(),
          predicate: m.get(3).unwrap().as_str().to_string(),
          data: m.get(4).unwrap().as_str().to_string(),
          data_type: match m.get(5) {
            None => None,
            Some(g3) => Some(g3.as_str().to_string()),
          },
        })
      }
      StringOrObject::Obj(o) => {
        let n = REG_CLASS_ID
          .captures(&o.class_id)
          .expect("class id should follow the format: <class_name>:<id_no>");

        Ok(InputLiteralNode {
          class_id: n.get(1).unwrap().as_str().to_string(),
          class_name: n.get(2).unwrap().as_str().to_string(),
          predicate: o.predicate,
          data: o.data,
          data_type: o.data_type,
        })
      }
    }
  }
}
