use crate::writers::stream_writer::StreamClassWriter;
use readers::prelude::{RAReader, Value, Index};
use crate::alignments::AlignmentFunc;
use crate::execution_plans::classes_map_plan::object_prop::ObjectProp;

pub fn generic_optional_oprop_map(readers: &[Box<dyn RAReader>], writer: &mut dyn StreamClassWriter, oplan: &ObjectProp, oalign: &mut AlignmentFunc, subj_id: &str, subj_val: &Value, subj_idx: &[Index], o_idx: &mut [Index], is_subj_blank: bool, is_new_subj: bool) {
  match oalign {
    AlignmentFunc::Single(f) => {
      match oplan {
        ObjectProp::BlankObject(blank_oplan) => {
          let uo_idx = f.align(subj_idx, subj_val, o_idx);
          // check if it has missing values or not, if yes, when we skip it
          // if target is optional, then, we won't have any missing object
          if !blank_oplan.is_target_optional && !writer.has_written_record(
                blank_oplan.class_id, &blank_oplan.pseudo_id.get_id_string(uo_idx)) {
            return;
          }
          writer.write_object_property(
            blank_oplan.class_id, subj_id,
            blank_oplan.predicate_id, &blank_oplan.pseudo_id.get_id_string(uo_idx), is_subj_blank, true, is_new_subj);
        }
        ObjectProp::IDObject(id_oplan) => {
          let uo_idx = f.align(subj_idx, subj_val, o_idx);
          let oval = readers[id_oplan.attribute.resource_id].get_value(uo_idx, 0);

          if id_oplan.is_target_optional {
            // target is optional, however, we need to check if uri is missing or not
            if id_oplan.missing_values.contains(oval) {
              // use pseudo id
              writer.write_object_property(
                id_oplan.class_id, subj_id,
                id_oplan.predicate_id,
                &id_oplan.pseudo_id.get_id_string(uo_idx), is_subj_blank, true, is_new_subj);
            } else {
              writer.write_object_property(
                id_oplan.class_id, subj_id,
                id_oplan.predicate_id,
                oval.as_str(), is_subj_blank, false, is_new_subj);
            }
          } else {
            // target is non-optional, so we have to check if the object has been written or not.
            // if it is the missing values, we use pseudo id, otherwise, we use the URI
            if id_oplan.missing_values.contains(oval) {
              // we need to use pseudo id
              let oid = id_oplan.pseudo_id.get_id_string(uo_idx);
              if writer.has_written_record(id_oplan.class_id, &oid) {
                // only write when it has been written
                writer.write_object_property(
                  id_oplan.class_id, subj_id,
                  id_oplan.predicate_id, &oid, is_subj_blank, true, is_new_subj);
              }
            } else {
              let oid = oval.as_str();
              if writer.has_written_record(id_oplan.class_id, oid) {
                // only write when it has been written
                writer.write_object_property(
                  id_oplan.class_id, subj_id,
                  id_oplan.predicate_id, oid, is_subj_blank, false, is_new_subj);
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
            // the target is optional, we always have the object
            let mut oiter = f.iter_alignments(subj_idx, subj_val, o_idx);
            loop {
              // no missing values, we just write that without checking
              writer.write_object_property(
                blank_oplan.class_id, subj_id, blank_oplan.predicate_id,
                  &blank_oplan.pseudo_id.get_id_string(oiter.value()), is_subj_blank, true, is_new_subj);
              if !oiter.advance() {
                break;
              }
            }
          } else {
            let mut oiter = f.iter_alignments(subj_idx, subj_val, o_idx);
            loop {
              // it has missing values, and we only write the record when it has been written
              if writer.has_written_record(
                    blank_oplan.class_id, &blank_oplan.pseudo_id.get_id_string(oiter.value())) {
                writer.write_object_property(
                  blank_oplan.class_id, subj_id, blank_oplan.predicate_id,
                  &blank_oplan.pseudo_id.get_id_string(oiter.value()), is_subj_blank, true, is_new_subj);
              }
              if !oiter.advance() {
                break;
              }
            }
          }
        }
        ObjectProp::IDObject(id_oplan) => {
          if id_oplan.is_target_optional {
            // target is optional, we always have target objects
            let mut oiter = f.iter_alignments(subj_idx, subj_val, o_idx);
            loop {
              let oval = readers[id_oplan.attribute.resource_id].get_value(oiter.value(), 0);
              // however, need to check if we have to use pseudo id (missing uri) or not
              if id_oplan.missing_values.contains(oval) {
                writer.write_object_property(
                  id_oplan.class_id,  subj_id, id_oplan.predicate_id,
                  &id_oplan.pseudo_id.get_id_string(oiter.value()), is_subj_blank, true, is_new_subj);
              } else {
                writer.write_object_property(
                  id_oplan.class_id,  subj_id, id_oplan.predicate_id,
                  oval.as_str(), is_subj_blank, false, is_new_subj);
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
                  writer.write_object_property(
                  id_oplan.class_id, subj_id, id_oplan.predicate_id,
                  &oid, is_subj_blank, true, is_new_subj);
                }
              } else {
                let oid = oval.as_str();
                if writer.has_written_record(id_oplan.class_id, oid) {
                  // only write when it has been written
                  writer.write_object_property(
                  id_oplan.class_id, subj_id, id_oplan.predicate_id,
                  &oid, is_subj_blank, false, is_new_subj);
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
}