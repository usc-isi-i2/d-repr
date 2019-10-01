use crate::models::*;
use fnv::FnvHashMap;
use serde::Deserialize;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct InputResources {
  pub resources: FnvHashMap<String, Resource>,
}

impl<'de> Deserialize<'de> for InputResources {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(untagged)]
    #[allow(non_camel_case_types)]
    enum InputResource_StringOrMap {
      Str(String),
      MapStr(FnvHashMap<String, String>),
      Map(FnvHashMap<String, Resource>),
    }

    let resources = match InputResource_StringOrMap::deserialize(deserializer)? {
      InputResource_StringOrMap::Str(rtype) => {
        let mut map = FnvHashMap::<String, Resource>::default();
        map.insert("default".to_string(), rtype2resource(&rtype));
        map
      }
      InputResource_StringOrMap::MapStr(map) => map
        .into_iter()
        .map(|(k, rtype)| (k, rtype2resource(&rtype)))
        .collect(),
      InputResource_StringOrMap::Map(map) => map,
    };

    Ok(InputResources { resources })
  }
}

fn rtype2resource(rtype: &str) -> Resource {
  match rtype.to_lowercase().as_str() {
    "csv" => Resource::CSV(CSVResource {
      delimiter: ",".to_string(),
    }),
    "json" => Resource::JSON,
    s => panic!("Invalid resource type: {}", s),
  }
}
