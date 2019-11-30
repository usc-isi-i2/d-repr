use crate::writers::stream_writer::StreamClassWriter;
use readers::prelude::{RAReader, Value, Index};
use crate::alignments::AlignmentFunc;
use crate::execution_plans::classes_map_plan::data_prop::DataProp;

/// optional
pub fn generic_optional_dprop_map(readers: &[Box<dyn RAReader>], writer: &mut dyn StreamClassWriter, dplan: &DataProp, dalign: &mut AlignmentFunc, subj_id: &str, subj_val: &Value, subj_idx: &[Index], d_idx: &mut [Index]) {
  match dalign {
    AlignmentFunc::Single(f) => {
      let dval = readers[dplan.attribute.resource_id].get_value(f.align(subj_idx, subj_val, d_idx), 0);
      if dplan.missing_values.len() > 0 && dval.is_hashable() && dplan.missing_values.contains(dval) {
        // checking if the missing values is > 0 to prevent hashing float values
        return;
      }
      writer.write_data_property(subj_id, dplan.predicate_id, dval);
    }
    AlignmentFunc::Multiple(f) => {
      if dplan.missing_values.len() > 0 {
        let mut diter = f.iter_alignments(subj_idx, subj_val, d_idx);
        loop {
          let dval = readers[dplan.attribute.resource_id].get_value(diter.value(), 0);
          if dval.is_hashable() && !dplan.missing_values.contains(dval) {
            writer.write_data_property(subj_id, dplan.predicate_id, dval);
          }
          if !diter.advance() {
            break;
          }
        }
      } else {
        let mut diter = f.iter_alignments(subj_idx, subj_val, d_idx);
        loop {
          let dval = readers[dplan.attribute.resource_id].get_value(diter.value(), 0);
          writer.write_data_property(subj_id, dplan.predicate_id, dval);
          if !diter.advance() {
            break;
          }
        }
      }
    }
  }
}
