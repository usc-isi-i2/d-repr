#[macro_export]
macro_rules! dprop_optional_map {
  // parameters:
  // 1. readers: &[Box<dyn RAReader>]
  // 2. writer: &mut dyn StreamClassWriter
  // 3. subj_id: &str
  // 4. subj_val: &Value
  // 5. subj_idx: &[Index]
  // 6. d_idx: &mut [Index]
  // 7. dplan: &DataProp
  // 8. dalign_func: &mut Box<SAlignmentFunc>
  // 9. dalign_func_type: single | multiple
  (
    $reader:expr, $writer:expr,
    $subj_id:expr, $subj_val:expr, $subj_idx:expr,
    $d_idx:expr, $dplan:expr, $dalign_func:expr,
    $dalign_func_type:ident
    $(, no_missing_values $no_missing_values:literal )?
  ) => {
    match_align_func!($dalign_func_type {
      single => {
        // we know that it is always single, but want to keep the code
        let dval = $reader.get_value($dalign_func.align($subj_idx, $subj_val, $d_idx), 0);
        mif!(dval.is_hashable() && !$dplan.missing_values.contains(dval); $( $no_missing_values ;)? {
          $writer.write_data_property($subj_id, $dplan.predicate_id, dval);
        })
      }
      multiple => {
        let mut diter = $dalign_func.iter_alignments($subj_idx, $subj_val, $d_idx);
        loop {
          let dval = $reader.get_value(diter.value(), 0);
          mif!(dval.is_hashable() && !$dplan.missing_values.contains(dval); $( $no_missing_values ;)? {
            $writer.write_data_property($subj_id, $dplan.predicate_id, dval);
          })
          if !diter.advance() {
            break;
          }
        }
      }
    })
  }
}