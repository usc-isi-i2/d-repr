use crate::writers::stream_writer::StreamClassWriter;
use readers::prelude::{RAReader};
use crate::alignments::func_builder::build_align_func;
use crate::executors::classes_map::buffer_writer::BufferWriter;
use crate::execution_plans::classes_map_plan::subject::Subject;
use crate::executors::classes_map::generic_algo::{generic_optional_dprop_map, generic_optional_oprop_map, generic_optional_buffered_oprop_map, generic_mandatory_dprop_map, generic_mandatory_oprop_map};
use crate::lang::Description;
use crate::execution_plans::ClassMapPlan;

/// Execute mapping for just one class. Handle all cases.
pub fn generic_class_map(readers: &[Box<dyn RAReader>], cls_writer: &mut dyn StreamClassWriter, desc: &Description, class_plan: &ClassMapPlan) {
  let mut dprop_aligns = class_plan.data_props.iter()
    .map(|a| build_align_func(&readers, desc, &a.alignments))
    .collect::<Vec<_>>();
  let mut dprop_indices = class_plan.data_props.iter()
    .map(|p| p.attribute.path.get_initial_step(readers[p.attribute.resource_id].as_ref()))
    .collect::<Vec<_>>();

  let mut oprop_aligns = class_plan.object_props.iter()
    .map(|a| build_align_func(&readers, desc, a.get_alignments()))
    .collect::<Vec<_>>();
  let mut oprop_indices = class_plan.object_props.iter()
    .map(|a| a.get_attr().path.get_initial_step(readers[a.get_attr().resource_id].as_ref()))
    .collect::<Vec<_>>();
  
  let mut buffered_oprop_aligns = class_plan.buffered_object_props.iter()
    .map(|a| build_align_func(&readers, desc, a.get_alignments()))
    .collect::<Vec<_>>();
  let mut buffered_oprop_indices = class_plan.buffered_object_props.iter()
    .map(|a| a.get_attr().path.get_initial_step(readers[a.get_attr().resource_id].as_ref()))
    .collect::<Vec<_>>();
  
  let mut external_subj = if let Subject::ExternalIDSubject(subj) = &class_plan.subject {
    Some((
      subj.real_id.0.path.get_initial_step(readers[subj.real_id.0.resource_id].as_ref()),
      build_align_func(&readers, desc, &subj.real_id.1).into_single()
    ))
  } else {
    None
  };
  let subj_attr = class_plan.subject.get_attr();
  let mut subj_iter = readers[subj_attr.resource_id].iter_index(&subj_attr.path);
  
  // not handle the third case
  assert!(class_plan.buffered_object_props.iter().all(|op| op.is_optional()));
  
  if class_plan.data_props.iter().any(|dp| !dp.is_optional) ||
    class_plan.object_props.iter().any(|op| !op.is_optional()) {
    // missing values will lead to drop of the record
    // if a subject has missing values then we clear it
    let mut buf_writer = BufferWriter::with_capacity(class_plan.data_props.len(), class_plan.object_props.len());

    loop {
      buf_writer.clear();

      let subj_val = readers[subj_attr.resource_id].get_value(subj_iter.value(), 0);
      let mut is_subj_blank = false;
      let subj_id: String = match &class_plan.subject {
        Subject::BlankSubject(subj) => {
          is_subj_blank = true;
          subj.pseudo_id.get_id_string(subj_iter.value())
        }
        Subject::InternalIDSubject(subj) => {
          if subj.missing_values.contains(subj_val) {
            if subj.is_optional {
              is_subj_blank = true;
              subj.pseudo_id.get_id_string(subj_iter.value())
            } else {
              // we have to skip it
              if !subj_iter.advance() {
                break;
              }
              continue;
            }
          } else {
            subj_val.as_str().to_string()
          }
        }
        Subject::ExternalIDSubject(subj) => {
          let esubj = external_subj.as_mut().unwrap();
          let idx = &mut esubj.0;
          esubj.1.align(subj_iter.value(), subj_val, idx);
          let real_id = readers[subj.real_id.0.resource_id].get_value(idx, 0);
          
          if subj.missing_values.contains(real_id) {
            if subj.is_optional {
              is_subj_blank = true;
              subj.pseudo_id.get_id_string(subj_iter.value())
            } else {
              // we have to skip it
              if !subj_iter.advance() {
                break;
              }
              continue;
            }
          } else {
            real_id.as_str().to_string()
          }
        }
      };

      let is_new_subject = !cls_writer.has_written_record(class_plan.class_id, &subj_id);
      let mut should_keep_record = true;

      if is_new_subject {
        for (di, dplan) in class_plan.data_props.iter().enumerate() {
          should_keep_record = generic_mandatory_dprop_map(&readers, &mut buf_writer, dplan, &mut dprop_aligns[di], &subj_id, subj_val, subj_iter.value(), &mut dprop_indices[di]);
          if !should_keep_record {
            break;
          }
        }
      }

      if !should_keep_record {
        // we have to skip it
        if !subj_iter.advance() {
          break;
        }
        continue;
      }

      for (oi, oplan) in class_plan.object_props.iter().enumerate() {
        should_keep_record = generic_mandatory_oprop_map(
          &readers, &mut buf_writer, cls_writer,
          oplan, &mut oprop_aligns[oi], &subj_id, subj_val, subj_iter.value(), &mut oprop_indices[oi]);

        if !should_keep_record {
          break;
        }
      }

      if should_keep_record {
        // we write the record to writer
        cls_writer.begin_record(&subj_id, is_subj_blank);
        for &(pred_id, val) in buf_writer.data_props.iter() {
          cls_writer.write_data_property(&subj_id, pred_id, val);
        }

        for (target_cls_id, pred_id, object, is_object_blank) in buf_writer.object_props.iter() {
          cls_writer.write_object_property(*target_cls_id, &subj_id, *pred_id, object, is_subj_blank, *is_object_blank, is_new_subject);
        }

        for &(target_cls_id, pred_id, object, is_object_blank) in &buf_writer.borrow_object_props {
          cls_writer.write_object_property(target_cls_id, &subj_id, pred_id, object, is_subj_blank, is_object_blank, is_new_subject);
        }

        for lplan in class_plan.literal_props.iter() {
          cls_writer.write_data_property(&subj_id, lplan.predicate_id, &lplan.value);
        }
        cls_writer.end_record();
      }

      if !subj_iter.advance() {
        break;
      }
    }
  } else {
    loop {
      let subj_val = readers[subj_attr.resource_id].get_value(subj_iter.value(), 0);
      let mut is_subj_blank: bool = false;
      
      let subj_id: String = match &class_plan.subject {
        Subject::BlankSubject(subj) => {
          is_subj_blank = true;
          subj.pseudo_id.get_id_string(subj_iter.value())
        }
        Subject::InternalIDSubject(subj) => {
          if subj.missing_values.contains(subj_val) {
            is_subj_blank = true;
            subj.pseudo_id.get_id_string(subj_iter.value())
          } else {
            subj_val.as_str().to_string()
          }
        }
        Subject::ExternalIDSubject(subj) => {
          let esubj = external_subj.as_mut().unwrap();
          let idx = &mut esubj.0;
          esubj.1.align(subj_iter.value(), subj_val, idx);
          let real_id = readers[subj.real_id.0.resource_id].get_value(idx, 0);
          if subj.missing_values.contains(real_id) {
            is_subj_blank = true;
            subj.pseudo_id.get_id_string(subj_iter.value())
          } else {
            real_id.as_str().to_string()
          }
        }
      };
    
      let is_new_subject = if class_plan.buffered_object_props.len() > 0 {
        cls_writer.begin_partial_buffering_record(&subj_id, is_subj_blank)
      } else {
        cls_writer.begin_record(&subj_id, is_subj_blank)
      };
    
      if is_new_subject {
        for (di, dplan) in class_plan.data_props.iter().enumerate() {
          generic_optional_dprop_map(&readers, cls_writer, dplan, &mut dprop_aligns[di], &subj_id, subj_val, subj_iter.value(), &mut dprop_indices[di]);
        }
      
        for lplan in class_plan.literal_props.iter() {
          cls_writer.write_data_property(&subj_id, lplan.predicate_id, &lplan.value);
        }
      }
    
      for (oi, oplan) in class_plan.object_props.iter().enumerate() {
        generic_optional_oprop_map(
          readers, cls_writer, oplan, &mut oprop_aligns[oi], &subj_id,
          subj_val, subj_iter.value(), &mut oprop_indices[oi], is_subj_blank, is_new_subject);
      }
    
      for (oi, oplan) in class_plan.buffered_object_props.iter().enumerate() {
        generic_optional_buffered_oprop_map(
          readers, cls_writer, oplan, &mut buffered_oprop_aligns[oi], &subj_id,
          subj_val, subj_iter.value(), &mut buffered_oprop_indices[oi], is_new_subject);
      }
    
      if is_new_subject {
        if class_plan.buffered_object_props.len() > 0 {
          cls_writer.end_partial_buffering_record();
        } else {
          cls_writer.end_record();
        }
      }
    
      if !subj_iter.advance() {
        break;
      }
    }
  }
}
