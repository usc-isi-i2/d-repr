use serde::{Deserialize, Serialize};

/// Each resource is associated with a resource id
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum Resource {
  #[serde(rename = "csv")]
  CSV(CSVResource),
  #[serde(rename = "json")]
  JSON(usize),
  #[serde(rename = "spreadsheet")]
  Spreadsheet(usize),
  #[serde(rename = "netcdf4")]
  NetCDF4(usize),
  #[serde(rename = "np-dict")]
  NPDict(usize),
  #[serde(rename = "geotiff")]
  GeoTIFF(usize),
  #[serde(rename = "shapefile")]
  Shapefile(usize),
  #[serde(rename = "container")]
  Container(usize)
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CSVResource {
  pub resource_id: usize,
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