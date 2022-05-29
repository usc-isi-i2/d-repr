use readers::prelude::{CSVRAReader, JSONRAReader, RAReader, SpreadsheetRAReader};

use crate::execution_plans::classes_map_plan::class_map_plan::ClassMapExecStrategy;
use crate::execution_plans::classes_map_plan::write_plan::WritePlan;
use crate::execution_plans::ClassesMapExecutionPlan;
use crate::executors::classes_map::generic_algo::generic_class_map;
#[cfg(feature = "enable-exec-macro-cls-map")]
use crate::executors::classes_map::specific_algo::specific_class_map::specific_class_map;
use crate::executors::preprocessing::exec_preprocessing;
use crate::executors::{PhysicalOutput, PhysicalResource};
use crate::lang::{Description, Resource};
use crate::writers::stream_writer::stream_writer::{StreamWriterResult, WriteResult};
use crate::writers::stream_writer::GraphPyWriter;
use crate::writers::stream_writer::OutputFormat;
use crate::writers::stream_writer::{GraphJSONWriter, TTLStreamWriter};

mod buffer_writer;
mod generic_algo;

#[cfg(feature = "enable-exec-macro-cls-map")]
pub mod specific_algo;

pub fn classes_map(
  resource_files: &[PhysicalResource],
  desc: &Description,
  plan: &mut ClassesMapExecutionPlan,
  output: &PhysicalOutput,
) -> WriteResult {
  let mut readers: Vec<Box<dyn RAReader>> = Vec::with_capacity(resource_files.len());
  for (i, resource) in desc.resources.iter().enumerate() {
    match resource {
      Resource::CSV(r) => {
        let reader = match &resource_files[i] {
          PhysicalResource::File(fpath) => {
            Box::new(CSVRAReader::from_file(fpath, r.get_delimiter()))
          }
          PhysicalResource::String(content) => {
            Box::new(CSVRAReader::from_str(content, r.get_delimiter()))
          }
        };
        readers.push(reader);
      }
      Resource::Spreadsheet(_) => {
        let reader = match &resource_files[i] {
          PhysicalResource::File(fpath) => Box::new(SpreadsheetRAReader::from_file(fpath)),
          _ => {
            unimplemented!("Haven't implemented reading spreadsheet from string yet")
          }
        };
        readers.push(reader);
      }
      Resource::JSON(_) => {
        let reader = match &resource_files[i] {
          PhysicalResource::File(fpath) => Box::new(JSONRAReader::from_file(fpath)),
          PhysicalResource::String(content) => Box::new(JSONRAReader::from_str(content)),
        };
        readers.push(reader);
      }
      Resource::NPDict(_) => {
        let reader = match &resource_files[i] {
          PhysicalResource::File(fpath) => Box::new(JSONRAReader::from_file(fpath)),
          PhysicalResource::String(content) => Box::new(JSONRAReader::from_str(content)),
        };
        readers.push(reader);
      }
      _ => unimplemented!(),
    }
  }
  exec_preprocessing(&mut readers, &desc.preprocessing);
  match &mut plan.write_plan {
    WritePlan::SingleWriter2File { class_write_modes } => {
      let mut writer: Box<dyn StreamWriterResult> = match output {
        PhysicalOutput::File {
          fpath,
          format: OutputFormat::TTL,
        } => Box::new(TTLStreamWriter::write2file(&fpath, &desc.semantic_model)),
        PhysicalOutput::File {
          fpath,
          format: OutputFormat::GraphJSON,
        } => Box::new(GraphJSONWriter::write2file(
          &format!("{}.node", fpath),
          &format!("{}.edge", fpath),
          &desc.semantic_model,
        )),
        PhysicalOutput::File {
          fpath: _,
          format: OutputFormat::GraphPy,
        } => {
          unimplemented!()
        }
        PhysicalOutput::Memory {
          format: OutputFormat::TTL,
        } => Box::new(TTLStreamWriter::write2str(&desc.semantic_model)),
        PhysicalOutput::Memory {
          format: OutputFormat::GraphJSON,
        } => Box::new(GraphJSONWriter::write2str(&desc.semantic_model)),
        PhysicalOutput::Memory {
          format: OutputFormat::GraphPy,
        } => Box::new(GraphPyWriter::write2mem(&desc.semantic_model)),
      };
      writer.begin();
      for cls_plan in plan.class_map_plans.iter_mut() {
        let mut cls_writer =
          writer.begin_class(cls_plan.class_id, class_write_modes[cls_plan.class_id]);
        match &cls_plan.exec_strategy {
          ClassMapExecStrategy::Generic => {
            generic_class_map(&readers, cls_writer.as_mut(), desc, cls_plan);
          }
          ClassMapExecStrategy::Macro(_) => {
            #[cfg(feature = "enable-exec-macro-cls-map")]
            specific_class_map(&readers, cls_writer.as_mut(), desc, cls_plan);
          }
        }
      }
      writer.end();
      writer.extract_result()
    }
  }
}

//#[allow(non_snake_case)]
///// Encoding scheme
/////
///// Re<N>: number of resources
///// R<t|f>: does the links are all mandatory or not (opposite of is_optional)
///// M<t|f>: have missing values or not
///// S<b|i|e>: subject is either blank node (b), internal id (i), or external id (e)
///// O<b|i|ip>: object props are either blank node (b), id (i), or id with pseudo id (ip)
///// B<t|f>: true need to buffer the object locally
//pub fn exc_Res1___Opt_f___Miss_f___Subj_b___Obj_b___Buff_f(reader: Box<dyn RAReader>, class_plan: ClassMapPlan) {
//  let object_props = class_plan.object_props
//    .into_iter()
//    .map(|o| o.into_blank_object())
//    .collect::<Vec<_>>();
//
//  let dprop_aligns = class_plan.data_props.iter()
//    .map(|a| a.alignment)
//    .collect::<Vec<_>>();
//
//  let subject = class_plan.subject.into_blank_subject();
//
//  let mut subj_iter = reader.iter_index(&subject.attr.path);
////  let subj_pos = subject.attr.path.get_initial_step(&readers[subject.attr.resource_id]);
//
//  loop {
//    let subj_val = reader.get_value(subj_iter.value(), 0);
//
//    for (di, dplan) in class_plan.data_props.iter().enumerate() {
//      // align them and get the value
//      dval = reader.get_value(dprop_aligns[di].align(subj_iter.value(), dindex))
//      // write them down
//    }
//
//    for (oi, oplan) in class_plan.object_props.iter().enumerate() {
//      // again, we align them and get the value
//      // write them down
//    }
//
//    if !subj_iter.advance() {
//      break;
//    }
//  }
//}
