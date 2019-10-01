use std::ptr::NonNull;

use crate::inputs::{InputPhysicalResource, InputRepresentation};
use crate::models::*;
use crate::python::{PyExecutor, ReaderPtr};
use crate::readers::*;
use crate::writers::StreamStateWriter;

pub use self::v01::mapping::mapping;
pub use self::v01::planning::plans::create_mapping_plans;
use hashbrown::HashMap;

mod common;
mod v01;

pub fn exec_mapping<W: StreamStateWriter>(
  repr: InputRepresentation,
  physical_resources: &Vec<InputPhysicalResource>,
  writer: &mut W,
) {
  if physical_resources.len() == 1 {
    let rid = &physical_resources[0].resource_id;
    let fpath = &physical_resources[0].resource_file;
    let resource = &repr.resources.resources[rid];

    match resource {
      Resource::CSV(conf) => {
        if repr.should_have_multiple_readers() {
          let mut maps = HashMap::new();
          maps.insert(
            rid.clone(),
            CSVRAReader::from_file(fpath, conf.get_delimiter()).into_value(),
          );
          let mut reader = MultipleRAReader::new(maps);
          wrap_exec_mapping_core(repr, &mut reader, writer);
        } else {
          let mut reader = CSVRAReader::from_file(fpath, conf.get_delimiter());
          wrap_exec_mapping_core(repr, &mut reader, writer);
        }
      },
      Resource::JSON => {
        if repr.should_have_multiple_readers() {
          let mut maps = HashMap::new();
          maps.insert(
            rid.clone(),
            JSONRAReader::from_file(fpath).into_value(),
          );
          let mut reader = MultipleRAReader::new(maps);
          wrap_exec_mapping_core(repr, &mut reader, writer);
        } else {
          let mut reader = JSONRAReader::from_file(fpath);
          wrap_exec_mapping_core(repr, &mut reader, writer);
        }
      },
      Resource::Spreadsheet => {
        if repr.should_have_multiple_readers() {
          let mut maps = HashMap::new();
          maps.insert(
            rid.clone(),
            SpreadsheetRAReader::from_file(fpath).into_value(),
          );
          let mut reader = MultipleRAReader::new(maps);
          wrap_exec_mapping_core(repr, &mut reader, writer);
        } else {
          let mut reader = SpreadsheetRAReader::from_file(fpath);
          wrap_exec_mapping_core(repr, &mut reader, writer);
        }
      }
      Resource::NetCDF => {
        if repr.should_have_multiple_readers() {
          let mut maps = HashMap::new();
          maps.insert(
            rid.clone(),
            NetCDFRAReader::from_file(fpath).into_value(),
          );
          let mut reader = MultipleRAReader::new(maps);
          wrap_exec_mapping_core(repr, &mut reader, writer);
        } else {
          let mut reader = NetCDFRAReader::from_file(fpath);
          wrap_exec_mapping_core(repr, &mut reader, writer);
        }
      }
    }
  } else {
    let mut maps: HashMap<String, Value> = HashMap::default();
    for res in physical_resources {
      let fpath = &res.resource_file;
      match &repr.resources.resources[&res.resource_id] {
        Resource::CSV(conf) => {
          maps.insert(
            res.resource_id.clone(),
            CSVRAReader::from_file(fpath, conf.get_delimiter()).into_value(),
          );
        }
        Resource::JSON => {
          maps.insert(
            res.resource_id.clone(),
            JSONRAReader::from_file(fpath).into_value(),
          );
        }
        Resource::Spreadsheet => {
          maps.insert(
            res.resource_id.clone(),
            SpreadsheetRAReader::from_file(fpath).into_value()
          );
        }
        Resource::NetCDF => {
          maps.insert(
            res.resource_id.clone(),
            NetCDFRAReader::from_file(fpath).into_value()
          );
        }
      }
    }

    let mut reader = MultipleRAReader::new(maps);
    wrap_exec_mapping_core(repr, &mut reader, writer);
  }
}

#[inline]
fn wrap_exec_mapping_core<R, W>(input_repr: InputRepresentation, reader: &mut R, writer: &mut W)
where
  R: 'static + RAReader,
  W: StreamStateWriter,
{
  // TODO: should we check if we need to init python or not? will it be faster?
  let mut py_executor = PyExecutor::new(ReaderPtr(
    NonNull::new(reader as *mut dyn RAReader).unwrap(),
  ));
  let mut repr = input_repr.into_repr(&mut py_executor);
  exec_mapping_core(&mut repr, reader, writer, &mut py_executor)
}

pub fn exec_mapping_core<R, W>(repr: &mut Representation, reader: &mut R, writer: &mut W, py_executor: &mut PyExecutor)
where
  R: 'static + RAReader,
  W: StreamStateWriter,
{
  writer.init(&repr.semantic_model);

  // run the preprocessing functions
  for func in &mut repr.preprocess_funcs {
    func.exec(py_executor, reader);
  }

  // clean the variables
  for var in &mut repr.variables {
    reader.ground_location(&mut var.location, 0);
  }

  // need to clean the data
  let plans = create_mapping_plans(reader, &repr);
  mapping(reader, writer, &plans);
}
