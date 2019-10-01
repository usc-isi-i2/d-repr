#[macro_export]
macro_rules! oprop_optional_map {
  (
    $reader:expr, $writer:expr, $oplan:expr,
    $oalign_func:expr, $subj_id:expr, $subj_val:expr, $subj_idx:expr,
    $o_idx:expr, $is_subj_blank:expr, $is_new_subj:expr, $oalign_type:ident, $oplan_type:ident
    $(, is_target_optional $is_target_optional:literal )?
    $(, no_missing_values $no_missing_values:literal )?
  ) => {
    match_align_func!($oalign_type {
      single => {
        match_object_prop!($oplan_type {
          blankobject => {
            let uo_idx = $oalign_func.align($subj_idx, $subj_val, $o_idx);
            let oid = $oplan.pseudo_id.get_id_string(uo_idx);
            // disable the if when !oplan.is_target_optional
            mif!($writer.has_written_record($oplan.class_id, &oid) $(; $is_target_optional )?; {
              $writer.write_object_property(
                $oplan.class_id, $subj_id, $oplan.predicate_id, &oid,
                $is_subj_blank, true, $is_new_subj);
            });
          }
          idobject => {
            let uo_idx = $oalign_func.align($subj_idx, $subj_val, $o_idx);
            let oval = $reader.get_value(uo_idx, 0);
            
            mif!($oplan.missing_values.contains(oval); $($no_missing_values exec_false_branch;)? {
              let oid = $oplan.pseudo_id.get_id_string(uo_idx);
              mif!($writer.has_written_record($oplan.class_id, &oid); $($is_target_optional;)? {
                $writer.write_object_property(
                  $oplan.class_id, $subj_id, $oplan.predicate_id, &oid,
                  $is_subj_blank, true, $is_new_subj)
              });
            } else {
              let oid = oval.as_str();
              mif!($writer.has_written_record($oplan.class_id, &oid); $($is_target_optional;)? {
                $writer.write_object_property(
                  $oplan.class_id, $subj_id, $oplan.predicate_id, &oid,
                  $is_subj_blank, false, $is_new_subj)
              });
            });
          }
        })
      }
      multiple => {
        match_object_prop!($oplan_type {
          blankobject => {
            let mut oiter = $oalign_func.iter_alignments($subj_idx, $subj_val, $o_idx);
            loop {
              let oid = $oplan.pseudo_id.get_id_string(oiter.value());
              // disable the if when !oplan.is_target_optional
              mif!($writer.has_written_record($oplan.class_id, &oid) $(; $is_target_optional )?; {
                $writer.write_object_property(
                  $oplan.class_id, $subj_id, $oplan.predicate_id, &oid,
                  $is_subj_blank, true, $is_new_subj);
              });
              if !oiter.advance() {
                break;
              }
            }
          }
          idobject => {
            let mut oiter = $oalign_func.iter_alignments($subj_idx, $subj_val, $o_idx);
            loop {
              let oval = $reader.get_value(oiter.value(), 0);
              mif!($oplan.missing_values.contains(oval); $($no_missing_values exec_false_branch;)? {
                let oid = $oplan.pseudo_id.get_id_string(oiter.value());
                mif!($writer.has_written_record($oplan.class_id, &oid); $($is_target_optional;)? {
                  $writer.write_object_property(
                    $oplan.class_id, $subj_id, $oplan.predicate_id, &oid,
                    $is_subj_blank, true, $is_new_subj)
                });
              } else {
                let oid = oval.as_str();
                mif!($writer.has_written_record($oplan.class_id, &oid); $($is_target_optional;)? {
                  $writer.write_object_property(
                    $oplan.class_id, $subj_id, $oplan.predicate_id, &oid,
                    $is_subj_blank, false, $is_new_subj)
                });
              });
              
              if !oiter.advance() {
                break;
              }
            }
          }
        })
      }
    })
  };
}

