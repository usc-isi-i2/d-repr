use crate::writers::stream_writer::StreamClassWriter;
use readers::prelude::{RAReader, Value, Index};
use crate::alignments::AlignmentFunc;
use crate::execution_plans::classes_map_plan::object_prop::ObjectProp;

pub fn generic_optional_buffered_oprop_map(readers: &[Box<dyn RAReader>], writer: &mut dyn StreamClassWriter, oplan: &ObjectProp, oalign: &mut AlignmentFunc, _subj_id: &str, subj_val: &Value, subj_idx: &[Index], o_idx: &mut [Index], _is_new_subj: bool) {
  // the only difference between this buffer oprop with non-buffer is the writer function (buffer_object_property instead of write_object_property), and
  // we cannot check if the record has been written or not, even if the target is non optional, and the subject is missing (because the subject may be optional, and the other property are required)
  match oalign {
    AlignmentFunc::Single(f) => {
      match oplan {
        ObjectProp::BlankObject(blank_oplan) => {
          let uo_idx = f.align(subj_idx, subj_val, o_idx);
          writer.buffer_object_property(
            blank_oplan.class_id,
            blank_oplan.predicate_id, blank_oplan.pseudo_id.get_id_string(uo_idx), true);
        }
        ObjectProp::IDObject(id_oplan) => {
          let uo_idx = f.align(subj_idx, subj_val, o_idx);
          let oval = readers[id_oplan.attribute.resource_id].get_value(uo_idx, 0);

          if id_oplan.missing_values.contains(oval) {
            // use pseudo id
            writer.buffer_object_property(
              id_oplan.class_id,
              id_oplan.predicate_id,
              id_oplan.pseudo_id.get_id_string(uo_idx), true);
          } else {
            writer.buffer_object_property(
              id_oplan.class_id,
              id_oplan.predicate_id,
              oval.as_str().to_string(), false);
          }
        }
      }
    }
    AlignmentFunc::Multiple(f) => {
      match oplan {
        ObjectProp::BlankObject(blank_oplan) => {
          // the target is optional, we always have the object
          let mut oiter = f.iter_alignments(subj_idx, subj_val, o_idx);
          loop {
            // no missing values, we just write that without checking
            writer.buffer_object_property(
              blank_oplan.class_id,  blank_oplan.predicate_id,
                blank_oplan.pseudo_id.get_id_string(oiter.value()), true);
            if !oiter.advance() {
              break;
            }
          }
        }
        ObjectProp::IDObject(id_oplan) => {
          let mut oiter = f.iter_alignments(subj_idx, subj_val, o_idx);
          loop {
            let oval = readers[id_oplan.attribute.resource_id].get_value(oiter.value(), 0);
            // need to check if we have to use pseudo id (missing uri) or not
            if id_oplan.missing_values.contains(oval) {
              writer.buffer_object_property(
                id_oplan.class_id,  id_oplan.predicate_id,
                id_oplan.pseudo_id.get_id_string(oiter.value()), true);
            } else {
              writer.buffer_object_property(
                id_oplan.class_id,  id_oplan.predicate_id,
                oval.as_str().to_string(), false);
            }
            if !oiter.advance() {
              break;
            }
          }
        }
      }
    }
  }
}