macro_rules! wrapped_dprop_optional_map {
  (
    $di:expr, $dplan:expr,
    $readers:expr, $writer:expr, $subj_id:expr, $subj_val:expr, $subj_idx:tt,
    $dprop_indices:ident, $dprop_aligns:ident
    $(, no_missing_values $no_missing_values:literal )?
  ) => {
    dprop_optional_map!(
      $readers[$dplan.attribute.resource_id], $writer, &$subj_id, $subj_val, $subj_idx,
      &mut $dprop_indices[$di], $dplan, $dprop_aligns[$di], single
      $(, no_missing_values $no_missing_values )?
    );
  };
}

macro_rules! wrapped_oprop_optional_map {
  (
    $oi:expr, $oplan:expr,
    $readers:expr, $writer:expr, $oprop_aligns:ident,
    $oprop_indices:ident, $subj_id:ident, $subj_val:ident, $subj_idx:expr,
    $is_subj_blank:ident, $is_new_subj:ident, $oalign_type:ident, $oplan_type:ident
    $(, is_target_optional $is_target_optional:literal )?
    $(, no_missing_values $no_missing_values:literal )?
  ) => {
    oprop_optional_map!(
      $readers[$oplan.attribute.resource_id], $writer, $oplan,
      $oprop_aligns[$oi], &$subj_id, $subj_val, $subj_idx, &mut $oprop_indices[$oi], $is_subj_blank,
      $is_new_subj, $oalign_type, $oplan_type
      $(, is_target_optional $is_target_optional )?
      $(, no_missing_values $no_missing_values )?
    )
  }
}

macro_rules! class_optional_map {
  (
    $n_Ob_Fs_Tt_oprops:tt,
    $n_Ob_Fs_Tf_oprops:tt,
    $n_Ob_Fm_Tt_oprops:tt,
    $n_Ob_Fm_Tf_oprops:tt,
    $n_Oi_Fs_Tt_Mt_oprops:tt,
    $n_Oi_Fs_Tf_Mt_oprops:tt,
    $n_Oi_Fm_Tt_Mt_oprops:tt,
    $n_Oi_Fm_Tf_Mt_oprops:tt,
    $n_Oi_Fs_Tt_Mf_oprops:tt,
    $n_Oi_Fs_Tf_Mf_oprops:tt,
    $n_Oi_Fm_Tt_Mf_oprops:tt,
    $n_Oi_Fm_Tf_Mf_oprops:tt,

    $n_Mt_dprops:tt,
    $n_Mf_dprops:tt,

    $readers:ident, $writer:ident, $class_plan:ident,
    $subj:ident, $subj_type:ident,
    $external_subj:ident,
    
    $Mt_dprop_aligns:ident, $Mt_dprop_indices:ident,
    $Mf_dprop_aligns:ident, $Mf_dprop_indices:ident,
    
    $Ob_Fs_Tt_oprops:ident, $Ob_Fs_Tt_oprop_aligns:ident, $Ob_Fs_Tt_oprop_indices:ident,
    $Ob_Fs_Tf_oprops:ident, $Ob_Fs_Tf_oprop_aligns:ident, $Ob_Fs_Tf_oprop_indices:ident,
    $Ob_Fm_Tt_oprops:ident, $Ob_Fm_Tt_oprop_aligns:ident, $Ob_Fm_Tt_oprop_indices:ident,
    $Ob_Fm_Tf_oprops:ident, $Ob_Fm_Tf_oprop_aligns:ident, $Ob_Fm_Tf_oprop_indices:ident,
    $Oi_Fs_Tt_Mt_oprops:ident, $Oi_Fs_Tt_Mt_oprop_aligns:ident, $Oi_Fs_Tt_Mt_oprop_indices:ident,
    $Oi_Fs_Tf_Mt_oprops:ident, $Oi_Fs_Tf_Mt_oprop_aligns:ident, $Oi_Fs_Tf_Mt_oprop_indices:ident,
    $Oi_Fm_Tt_Mt_oprops:ident, $Oi_Fm_Tt_Mt_oprop_aligns:ident, $Oi_Fm_Tt_Mt_oprop_indices:ident,
    $Oi_Fm_Tf_Mt_oprops:ident, $Oi_Fm_Tf_Mt_oprop_aligns:ident, $Oi_Fm_Tf_Mt_oprop_indices:ident,
    $Oi_Fs_Tt_Mf_oprops:ident, $Oi_Fs_Tt_Mf_oprop_aligns:ident, $Oi_Fs_Tt_Mf_oprop_indices:ident,
    $Oi_Fs_Tf_Mf_oprops:ident, $Oi_Fs_Tf_Mf_oprop_aligns:ident, $Oi_Fs_Tf_Mf_oprop_indices:ident,
    $Oi_Fm_Tt_Mf_oprops:ident, $Oi_Fm_Tt_Mf_oprop_aligns:ident, $Oi_Fm_Tt_Mf_oprop_indices:ident,
    $Oi_Fm_Tf_Mf_oprops:ident, $Oi_Fm_Tf_Mf_oprop_aligns:ident, $Oi_Fm_Tf_Mf_oprop_indices:ident
    $( , always_new_subject $always_new_subject:literal )?
    $( , has_no_buffered_object_props $has_no_buffered_object_props:literal )?
    $( , no_missing_subj_values $no_missing_subj_values:literal )?
  ) => {
    let subj_attr = $class_plan.subject.get_attr();
    let mut subj_iter = $readers[subj_attr.resource_id].iter_index(&subj_attr.path);
    let mut subj_id: String;
    
    loop {
      let mut is_subj_blank = false;
      let subj_val = $readers[subj_attr.resource_id].get_value(subj_iter.value(), 0);
      
      match_subj_prop!($subj_type {
        blanksubject => {
          subj_id = $subj.pseudo_id.get_id_string(subj_iter.value());
        }
        internalidsubject => {
          mif!($subj.missing_values.contains(subj_val) $(; $no_missing_subj_values exec_false_branch )? ; {
            is_subj_blank = true;
            subj_id = $subj.pseudo_id.get_id_string(subj_iter.value());
          } else {
            subj_id = subj_val.as_str().to_string();
          });
        }
        externalidsubject => {
          let esubj = $external_subj.as_mut().unwrap();
          let idx = &mut esubj.0;
          esubj.1.align(subj_iter.value(), subj_val, idx);
          
          let real_id = $readers[$subj.real_id.0.resource_id].get_value(idx, 0);
          mif!($subj.missing_values.contains(real_id) $(; $no_missing_subj_values exec_false_branch )? ; {
            is_subj_blank = true;
            subj_id = $subj.pseudo_id.get_id_string(subj_iter.value())
          } else {
            subj_id = real_id.as_str().to_string()
          });
        }
      });
      
      let is_new_subject;
      exclusive_if!($class_plan.buffered_object_props.len() == 0; $( $has_no_buffered_object_props ; )? {
        is_new_subject = $writer.begin_record(&subj_id, true)
      } else {
        is_new_subject = $writer.begin_partial_buffering_record(&subj_id, true);
      });

      unroll_enumerate!(
        $Ob_Fs_Tt_oprops, $n_Ob_Fs_Tt_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Ob_Fs_Tt_oprop_aligns, $Ob_Fs_Tt_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, single, blankobject,
        is_target_optional true, no_missing_values true
      );
      unroll_enumerate!(
        $Ob_Fs_Tf_oprops, $n_Ob_Fs_Tf_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Ob_Fs_Tf_oprop_aligns, $Ob_Fs_Tf_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, single, blankobject
      );
      unroll_enumerate!(
        $Ob_Fm_Tt_oprops, $n_Ob_Fm_Tt_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Ob_Fm_Tt_oprop_aligns, $Ob_Fm_Tt_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, multiple, blankobject,
        is_target_optional true, no_missing_values true
      );
      unroll_enumerate!(
        $Ob_Fm_Tf_oprops, $n_Ob_Fm_Tf_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Ob_Fm_Tf_oprop_aligns, $Ob_Fm_Tf_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, multiple, blankobject
      );
      // ********
      unroll_enumerate!(
        $Oi_Fs_Tt_Mt_oprops, $n_Oi_Fs_Tt_Mt_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Oi_Fs_Tt_Mt_oprop_aligns, $Oi_Fs_Tt_Mt_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, single, idobject,
        is_target_optional true
      );
      unroll_enumerate!(
        $Oi_Fs_Tt_Mf_oprops, $n_Oi_Fs_Tt_Mf_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Oi_Fs_Tt_Mf_oprop_aligns, $Oi_Fs_Tt_Mf_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, single, idobject,
        is_target_optional true, no_missing_values true
      );
      unroll_enumerate!(
        $Oi_Fs_Tf_Mt_oprops, $n_Oi_Fs_Tf_Mt_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Oi_Fs_Tf_Mt_oprop_aligns, $Oi_Fs_Tf_Mt_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, single, idobject
      );
      unroll_enumerate!(
        $Oi_Fs_Tf_Mf_oprops, $n_Oi_Fs_Tf_Mf_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Oi_Fs_Tf_Mf_oprop_aligns, $Oi_Fs_Tf_Mf_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, single, idobject,
        no_missing_values true
      );
      // *******
      unroll_enumerate!(
        $Oi_Fm_Tt_Mt_oprops, $n_Oi_Fm_Tt_Mt_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Oi_Fm_Tt_Mt_oprop_aligns, $Oi_Fm_Tt_Mt_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, multiple, idobject,
        is_target_optional true
      );
      unroll_enumerate!(
        $Oi_Fm_Tt_Mf_oprops, $n_Oi_Fm_Tt_Mf_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Oi_Fm_Tt_Mf_oprop_aligns, $Oi_Fm_Tt_Mf_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, multiple, idobject,
        is_target_optional true, no_missing_values true
      );
      unroll_enumerate!(
        $Oi_Fm_Tf_Mt_oprops, $n_Oi_Fm_Tf_Mt_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Oi_Fm_Tf_Mt_oprop_aligns, $Oi_Fm_Tf_Mt_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, multiple, idobject
      );
      unroll_enumerate!(
        $Oi_Fm_Tf_Mf_oprops, $n_Oi_Fm_Tf_Mf_oprops,
        wrapped_oprop_optional_map,
        $readers, $writer, $Oi_Fm_Tf_Mf_oprop_aligns, $Oi_Fm_Tf_Mf_oprop_indices,
        subj_id, subj_val, (subj_iter.value()), is_subj_blank,
        is_new_subject, multiple, idobject,
        no_missing_values true
      );

      hide_block!($( $has_no_buffered_object_props ; )? {
        // we haven't implement oprop_buffer_optional_map yet, and this should be filtered out
        unreachable!();
      });
      
      mif!(is_new_subject; $( $always_new_subject; )? {
        unroll_enumerate!(
          $class_plan.data_props, $n_Mf_dprops, wrapped_dprop_optional_map,
          $readers, $writer, subj_id, subj_val, (subj_iter.value()),
          $Mf_dprop_indices, $Mf_dprop_aligns,
          no_missing_values true
        );
        unroll_enumerate!(
          $class_plan.data_props, $n_Mt_dprops, wrapped_dprop_optional_map,
          $readers, $writer, subj_id, subj_val, (subj_iter.value()),
          $Mt_dprop_indices, $Mt_dprop_aligns
        );
      
        for lplan in $class_plan.literal_props.iter() {
          $writer.write_data_property(&subj_id, lplan.predicate_id, &lplan.value);
        }
        
        exclusive_if!($class_plan.buffered_object_props.len() == 0; $( $has_no_buffered_object_props ; )? {
          $writer.end_record();
        } else {
          $writer.end_partial_buffering_record();
        });
      });
      
      if !subj_iter.advance() {
        break;
      }
    }
  };
}