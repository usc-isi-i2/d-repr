use regex::Regex;
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
pub struct InputRelation {
  pub source_id: String,
  pub source_name: String,
  pub target_id: String,
  pub target_name: String,
  pub predicate: String,
}

lazy_static! {
  static ref REG_RELATION: Regex = Regex::new(r"^((.+):\d+)--(.+)--((.+):\d+)$").unwrap();
  static ref REG_CLASS_ID: Regex = Regex::new(r"^((.+):\d+)$").unwrap();
}

impl<'de> Deserialize<'de> for InputRelation {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct TmpRelation {
      source_id: String,
      target_id: String,
      predicate: String,
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrObj {
      Str(String),
      Obj(TmpRelation),
    }

    match StringOrObj::deserialize(deserializer)? {
      StringOrObj::Str(s) => {
        let m = REG_RELATION.captures(&s).expect("class id should follow the format: <class_name>:<id_no>--<predicate>--<class_name>:<id_no>");

        Ok(InputRelation {
          source_id: m.get(1).unwrap().as_str().to_string(),
          source_name: m.get(2).unwrap().as_str().to_string(),
          target_id: m.get(4).unwrap().as_str().to_string(),
          target_name: m.get(5).unwrap().as_str().to_string(),
          predicate: m.get(3).unwrap().as_str().to_string(),
        })
      }
      StringOrObj::Obj(r) => {
        let s = REG_CLASS_ID.captures(&r.source_id).expect("shorthand for data node should follow the format: <class_name>:<id_no>--<predicate>(^^<data_type>)?");
        let t = REG_CLASS_ID.captures(&r.target_id).expect("shorthand for data node should follow the format: <class_name>:<id_no>--<predicate>(^^<data_type>)?");

        Ok(InputRelation {
          source_id: s.get(1).unwrap().as_str().to_string(),
          source_name: s.get(2).unwrap().as_str().to_string(),
          target_id: t.get(1).unwrap().as_str().to_string(),
          target_name: t.get(2).unwrap().as_str().to_string(),
          predicate: r.predicate,
        })
      }
    }
  }
}
