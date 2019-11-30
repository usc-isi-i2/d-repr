use readers::prelude::{Index, RAReader, Value};

use crate::alignments::AlignmentFunc;
use crate::execution_plans::classes_map_plan::data_prop::DataProp;
use crate::executors::classes_map::buffer_writer::BufferWriter;

/// non-optional, return if the record should be kept
pub fn generic_mandatory_dprop_map<'a>(readers: &'a [Box<dyn RAReader>], writer: &mut BufferWriter<'a>, dplan: &DataProp, dalign: &mut AlignmentFunc, _subj_id: &str, subj_val: &Value, subj_idx: &[Index], d_idx: &mut [Index]) -> bool {
  match dalign {
    AlignmentFunc::Single(f) => {
      let dval = readers[dplan.attribute.resource_id].get_value(f.align(subj_idx, subj_val, d_idx), 0);
      // check if it is the missing value, and keep the record or not depends on if the link is optional
      if dplan.missing_values.len() > 0 && dval.is_hashable() && dplan.missing_values.contains(dval) {
        // checking if the missing values is > 0 to prevent hashing float values
        return dplan.is_optional;
      }
      writer.write_data_property(dplan.predicate_id, dval);
    }
    AlignmentFunc::Multiple(f) => {
      if dplan.missing_values.len() > 0 {
        let mut diter = f.iter_alignments(subj_idx, subj_val, d_idx);
        loop {
          let dval = readers[dplan.attribute.resource_id].get_value(diter.value(), 0);
          if dval.is_hashable() && !dplan.missing_values.contains(dval) {
            // not missing value, write now
            writer.write_data_property(dplan.predicate_id, dval);
          } else if !dplan.is_optional {
            // it is the missing value, and if the link is not optional, we should not keep it
            return false;
          }
          
          if !diter.advance() {
            break;
          }
        }
      } else {
        // no missing value
        let mut diter = f.iter_alignments(subj_idx, subj_val, d_idx);
        loop {
          let dval = readers[dplan.attribute.resource_id].get_value(diter.value(), 0);
          writer.write_data_property(dplan.predicate_id, dval);
          if !diter.advance() {
            break;
          }
        }
      }
    }
  }
  
  return true;
}
