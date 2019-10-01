use crate::writers::stream_writer::StreamClassWriter;
use readers::prelude::{RAReader, Value, Index};
use crate::alignments::AlignmentFunc;
use crate::execution_plans::classes_map_plan::object_prop::ObjectProp;
use crate::executors::classes_map::buffer_writer::BufferWriter;

pub fn generic_mandatory_oprop_map<'a>(readers: &'a [Box<dyn RAReader>], buf_writer: &mut BufferWriter<'a>, writer: &mut dyn StreamClassWriter, oplan: &ObjectProp, oalign: &mut AlignmentFunc, _subj_id: &str, subj_val: &Value, subj_idx: &[Index], o_idx: &mut [Index]) -> bool {
  match oalign {
    AlignmentFunc::Single(f) => {
      match oplan {
        ObjectProp::BlankObject(blank_oplan) => {
          let uo_idx = f.align(subj_idx, subj_val, o_idx);
          // check if it has missing values or not, if yes, keep if only if it is optional
          if !blank_oplan.is_target_optional && !writer.has_written_record(
                blank_oplan.class_id, &blank_oplan.pseudo_id.get_id_string(uo_idx)) {
            return blank_oplan.is_optional;
          }
          
          buf_writer.write_object_property(
            blank_oplan.class_id, blank_oplan.predicate_id,
            blank_oplan.pseudo_id.get_id_string(uo_idx), true);
        }
        ObjectProp::IDObject(id_oplan) => {
          let uo_idx = f.align(subj_idx, subj_val, o_idx);
          let oval = readers[id_oplan.attribute.resource_id].get_value(uo_idx, 0);

          if id_oplan.is_target_optional {
            // target is optional, however, we need to check if uri is missing or not
            if id_oplan.missing_values.contains(oval) {
              buf_writer.write_object_property(
              id_oplan.class_id,
              id_oplan.predicate_id,
              id_oplan.pseudo_id.get_id_string(uo_idx), true);
            } else {
              buf_writer.write_borrow_object_property(
              id_oplan.class_id,
              id_oplan.predicate_id,
              oval.as_str(), false);
            };
          } else {
            // target is non-optional, so we have to check if the object has been written or not.
            // if it is the missing values, we use pseudo id, otherwise, we use the URI
            if id_oplan.missing_values.contains(oval) {
              let oid = id_oplan.pseudo_id.get_id_string(uo_idx);
              if writer.has_written_record(id_oplan.class_id, &oid) {
                buf_writer.write_object_property(
                id_oplan.class_id,
                id_oplan.predicate_id, oid, true);
              } else {
                return id_oplan.is_optional;
              }
            } else {
              let oid = oval.as_str();
              if writer.has_written_record(id_oplan.class_id, oid) {
                // only write when it has been written
                buf_writer.write_borrow_object_property(
                id_oplan.class_id,
                id_oplan.predicate_id, oid, false);
              } else {
                // only keep it if it is optional
                return id_oplan.is_optional;
              }
            }
          }
        }
      }
    }
    AlignmentFunc::Multiple(f) => {
      match oplan {
        ObjectProp::BlankObject(blank_oplan) => {
          if blank_oplan.is_target_optional {
            let mut oiter = f.iter_alignments(subj_idx, subj_val, o_idx);
            loop {
              // no missing values, we just write that without checking
              buf_writer.write_object_property(
                blank_oplan.class_id, blank_oplan.predicate_id,
                  blank_oplan.pseudo_id.get_id_string(oiter.value()), true);
              if !oiter.advance() {
                break;
              }
            }
          } else {
            let mut oiter = f.iter_alignments(subj_idx, subj_val, o_idx);
            loop {
              // it has missing values, and we only write the record when it has been written
              let oid = blank_oplan.pseudo_id.get_id_string(oiter.value());
              if writer.has_written_record(
                    blank_oplan.class_id, &oid) {
                buf_writer.write_object_property(
                  blank_oplan.class_id, blank_oplan.predicate_id,
                  oid, true);
              } else if !blank_oplan.is_optional {
                return false;
              }
              
              if !oiter.advance() {
                break;
              }
            }
          }
        }
        ObjectProp::IDObject(id_oplan) => {
          if id_oplan.is_target_optional {
            // target is optional, no need to check if it has been written
            let mut oiter = f.iter_alignments(subj_idx, subj_val, o_idx);
            loop {
              let oval = readers[id_oplan.attribute.resource_id].get_value(oiter.value(), 0);
              // however, need to check if uri is missing or not
              if id_oplan.missing_values.contains(oval) {
                buf_writer.write_object_property(
                  id_oplan.class_id, id_oplan.predicate_id,
                  id_oplan.pseudo_id.get_id_string(oiter.value()), true
                );
              } else {
                buf_writer.write_borrow_object_property(
                id_oplan.class_id, id_oplan.predicate_id,
                  oval.as_str(), false);
              }
              if !oiter.advance() {
                break;
              }
            }
          } else {
            // target is non-optional, we have to check if it has been written or not
            let mut oiter = f.iter_alignments(subj_idx, subj_val, o_idx);
            loop {
              let oval = readers[id_oplan.attribute.resource_id].get_value(oiter.value(), 0);
              // if we have to use pseudo id (missing uri) or not
              if id_oplan.missing_values.contains(oval) {
                let oid = id_oplan.pseudo_id.get_id_string(oiter.value());
                if writer.has_written_record(id_oplan.class_id, &oid) {
                  // only write when it has been written
                  buf_writer.write_object_property(
                  id_oplan.class_id, id_oplan.predicate_id,
                  oid, true);
                } else if !id_oplan.is_optional {
                  // non-optional, we discard immediately
                  return false;
                }
              } else {
                let oid = oval.as_str();
                if writer.has_written_record(id_oplan.class_id, oid) {
                  // only write when it has been written
                  buf_writer.write_borrow_object_property(
                  id_oplan.class_id, id_oplan.predicate_id,
                  &oid, false);
                } else if !id_oplan.is_optional {
                  // non-optional, we discard immediately
                  return false;
                }
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
  
  return true;
}