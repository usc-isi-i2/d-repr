use crate::execution_plans::subject::Subject;
use crate::execution_plans::ClassMapPlan;
use crate::lang::Description;
use crate::writers::stream_writer::StreamClassWriter;
use readers::prelude::{Index, RAReader};

use super::class_macro_map;
use crate::alignments::func_builder::build_align_func;
use crate::alignments::{MAlignmentFunc, SAlignmentFunc};
use crate::execution_plans::classes_map_plan::data_prop::DataProp;
use crate::execution_plans::classes_map_plan::object_prop::{BlankObject, IDObject, ObjectProp};

const max_Mt_dprops: usize = 10;
const max_Mf_dprops: usize = 20;
const max_Ob_Fs_Tt_oprops: usize = 2;
const max_Ob_Fs_Tf_oprops: usize = 0;
const max_Ob_Fm_Tt_oprops: usize = 2;
const max_Ob_Fm_Tf_oprops: usize = 0;
const max_Oi_Fs_Tt_Mt_oprops: usize = 2;
const max_Oi_Fs_Tf_Mt_oprops: usize = 0;
const max_Oi_Fm_Tt_Mt_oprops: usize = 2;
const max_Oi_Fm_Tf_Mt_oprops: usize = 0;
const max_Oi_Fs_Tt_Mf_oprops: usize = 2;
const max_Oi_Fs_Tf_Mf_oprops: usize = 0;
const max_Oi_Fm_Tt_Mf_oprops: usize = 2;
const max_Oi_Fm_Tf_Mf_oprops: usize = 0;

pub fn analyze_specific_algo_strategy(class_plan: &ClassMapPlan) -> Option<String> {
  // right now only handle all optional
  if class_plan.data_props.iter().any(|dp| !dp.is_optional)
    || class_plan.object_props.iter().any(|op| !op.is_optional())
  {
    return None;
  }
  // only handle non_buffering
  if class_plan.buffered_object_props.len() > 0 {
    return None;
  }

  // only handle for limited numbers of data properties and object properties
  let (Mt_dprops, Mf_dprops) = split_dprops(class_plan);
  let (
    Ob_Fs_Tt_oprops,
    Ob_Fs_Tf_oprops,
    Ob_Fm_Tt_oprops,
    Ob_Fm_Tf_oprops,
    Oi_Fs_Tt_Mt_oprops,
    Oi_Fs_Tf_Mt_oprops,
    Oi_Fm_Tt_Mt_oprops,
    Oi_Fm_Tf_Mt_oprops,
    Oi_Fs_Tt_Mf_oprops,
    Oi_Fs_Tf_Mf_oprops,
    Oi_Fm_Tt_Mf_oprops,
    Oi_Fm_Tf_Mf_oprops,
  ) = split_oprops(class_plan);

  if Mt_dprops.len() > max_Mt_dprops
    || Mf_dprops.len() > max_Mf_dprops
    || Ob_Fs_Tt_oprops.len() > max_Ob_Fs_Tt_oprops
    || Ob_Fs_Tf_oprops.len() > max_Ob_Fs_Tf_oprops
    || Ob_Fm_Tt_oprops.len() > max_Ob_Fm_Tt_oprops
    || Ob_Fm_Tf_oprops.len() > max_Ob_Fm_Tf_oprops
    || Oi_Fs_Tt_Mt_oprops.len() > max_Oi_Fs_Tt_Mt_oprops
    || Oi_Fs_Tf_Mt_oprops.len() > max_Oi_Fs_Tf_Mt_oprops
    || Oi_Fm_Tt_Mt_oprops.len() > max_Oi_Fm_Tt_Mt_oprops
    || Oi_Fm_Tf_Mt_oprops.len() > max_Oi_Fm_Tf_Mt_oprops
    || Oi_Fs_Tt_Mf_oprops.len() > max_Oi_Fs_Tt_Mf_oprops
    || Oi_Fs_Tf_Mf_oprops.len() > max_Oi_Fs_Tf_Mf_oprops
    || Oi_Fm_Tt_Mf_oprops.len() > max_Oi_Fm_Tt_Mf_oprops
    || Oi_Fm_Tf_Mf_oprops.len() > max_Oi_Fm_Tf_Mf_oprops
  {
    return None;
  }
  // disable
  return None;
  //  Some(format!(r#"
  //Run RDF Mapping using macro strategy. Overview:
  //* |Mt_dprops| = {Mt_dprops} (max = {max_Mt_dprops})
  //* |Mf_dprops| = {Mf_dprops} (max = {max_Mf_dprops})
  //* |Ob_Fs_Tt_oprops| = {Ob_Fs_Tt_oprops} (max = {max_Ob_Fs_Tt_oprops})
  //* |Ob_Fs_Tf_oprops| = {Ob_Fs_Tf_oprops} (max = {max_Ob_Fs_Tf_oprops})
  //* |Ob_Fm_Tt_oprops| = {Ob_Fm_Tt_oprops} (max = {max_Ob_Fm_Tt_oprops})
  //* |Ob_Fm_Tf_oprops| = {Ob_Fm_Tf_oprops} (max = {max_Ob_Fm_Tf_oprops})
  //* |Oi_Fs_Tt_Mt_oprops| = {Oi_Fs_Tt_Mt_oprops} (max = {max_Oi_Fs_Tt_Mt_oprops})
  //* |Oi_Fs_Tf_Mt_oprops| = {Oi_Fs_Tf_Mt_oprops} (max = {max_Oi_Fs_Tf_Mt_oprops})
  //* |Oi_Fm_Tt_Mt_oprops| = {Oi_Fm_Tt_Mt_oprops} (max = {max_Oi_Fm_Tt_Mt_oprops})
  //* |Oi_Fm_Tf_Mt_oprops| = {Oi_Fm_Tf_Mt_oprops} (max = {max_Oi_Fm_Tf_Mt_oprops})
  //* |Oi_Fs_Tt_Mf_oprops| = {Oi_Fs_Tt_Mf_oprops} (max = {max_Oi_Fs_Tt_Mf_oprops})
  //* |Oi_Fs_Tf_Mf_oprops| = {Oi_Fs_Tf_Mf_oprops} (max = {max_Oi_Fs_Tf_Mf_oprops})
  //* |Oi_Fm_Tt_Mf_oprops| = {Oi_Fm_Tt_Mf_oprops} (max = {max_Oi_Fm_Tt_Mf_oprops})
  //* |Oi_Fm_Tf_Mf_oprops| = {Oi_Fm_Tf_Mf_oprops} (max = {max_Oi_Fm_Tf_Mf_oprops})
  //  "#,
  //    Mt_dprops=Mt_dprops.len(), max_Mt_dprops=max_Mt_dprops,
  //    Mf_dprops=Mf_dprops.len(), max_Mf_dprops=max_Mf_dprops,
  //    Ob_Fs_Tt_oprops=Ob_Fs_Tt_oprops.len(), max_Ob_Fs_Tt_oprops=max_Ob_Fs_Tt_oprops,
  //    Ob_Fs_Tf_oprops=Ob_Fs_Tf_oprops.len(), max_Ob_Fs_Tf_oprops=max_Ob_Fs_Tf_oprops,
  //    Ob_Fm_Tt_oprops=Ob_Fm_Tt_oprops.len(), max_Ob_Fm_Tt_oprops=max_Ob_Fm_Tt_oprops,
  //    Ob_Fm_Tf_oprops=Ob_Fm_Tf_oprops.len(), max_Ob_Fm_Tf_oprops=max_Ob_Fm_Tf_oprops,
  //    Oi_Fs_Tt_Mt_oprops=Oi_Fs_Tt_Mt_oprops.len(), max_Oi_Fs_Tt_Mt_oprops=max_Oi_Fs_Tt_Mt_oprops,
  //    Oi_Fs_Tf_Mt_oprops=Oi_Fs_Tf_Mt_oprops.len(), max_Oi_Fs_Tf_Mt_oprops=max_Oi_Fs_Tf_Mt_oprops,
  //    Oi_Fm_Tt_Mt_oprops=Oi_Fm_Tt_Mt_oprops.len(), max_Oi_Fm_Tt_Mt_oprops=max_Oi_Fm_Tt_Mt_oprops,
  //    Oi_Fm_Tf_Mt_oprops=Oi_Fm_Tf_Mt_oprops.len(), max_Oi_Fm_Tf_Mt_oprops=max_Oi_Fm_Tf_Mt_oprops,
  //    Oi_Fs_Tt_Mf_oprops=Oi_Fs_Tt_Mf_oprops.len(), max_Oi_Fs_Tt_Mf_oprops=max_Oi_Fs_Tt_Mf_oprops,
  //    Oi_Fs_Tf_Mf_oprops=Oi_Fs_Tf_Mf_oprops.len(), max_Oi_Fs_Tf_Mf_oprops=max_Oi_Fs_Tf_Mf_oprops,
  //    Oi_Fm_Tt_Mf_oprops=Oi_Fm_Tt_Mf_oprops.len(), max_Oi_Fm_Tt_Mf_oprops=max_Oi_Fm_Tt_Mf_oprops,
  //    Oi_Fm_Tf_Mf_oprops=Oi_Fm_Tf_Mf_oprops.len(), max_Oi_Fm_Tf_Mf_oprops=max_Oi_Fm_Tf_Mf_oprops
  //  ))
}

pub fn specific_class_map(
  readers: &[Box<dyn RAReader>],
  cls_writer: &mut dyn StreamClassWriter,
  desc: &Description,
  class_plan: &ClassMapPlan,
) -> bool {
  assert!(class_plan.exec_strategy.is_macro());
  // Encoding scheme
  //
  // R<t|f>: does the links are all mandatory or not (opposite of is_optional)
  // M<t|f>: have missing values (t) or not (f)
  // O<b|i>: object props are either blank node (b), id (i)
  // F<s|m>: alignment function type is either single alignment (s) or multiple (m)
  // T<t|f>: whether the target object prop is optional (t) or not (f)
  let (Mt_dprops, Mf_dprops) = split_dprops(class_plan);
  let (
    Ob_Fs_Tt_oprops,
    Ob_Fs_Tf_oprops,
    Ob_Fm_Tt_oprops,
    Ob_Fm_Tf_oprops,
    Oi_Fs_Tt_Mt_oprops,
    Oi_Fs_Tf_Mt_oprops,
    Oi_Fm_Tt_Mt_oprops,
    Oi_Fm_Tf_Mt_oprops,
    Oi_Fs_Tt_Mf_oprops,
    Oi_Fs_Tf_Mf_oprops,
    Oi_Fm_Tt_Mf_oprops,
    Oi_Fm_Tf_Mf_oprops,
  ) = split_oprops(class_plan);
  let (mut Mt_dprop_aligns, mut Mt_dprop_indices) = get_dalign_and_index(readers, desc, &Mt_dprops);
  let (mut Mf_dprop_aligns, mut Mf_dprop_indices) = get_dalign_and_index(readers, desc, &Mf_dprops);

  let (mut Ob_Fs_Tt_oprop_aligns, mut Ob_Fs_Tt_oprop_indices) =
    get_Ob_Fs_align_and_index(readers, desc, &Ob_Fs_Tt_oprops);
  let (mut Ob_Fs_Tf_oprop_aligns, mut Ob_Fs_Tf_oprop_indices) =
    get_Ob_Fs_align_and_index(readers, desc, &Ob_Fs_Tf_oprops);
  let (mut Ob_Fm_Tt_oprop_aligns, mut Ob_Fm_Tt_oprop_indices) =
    get_Ob_Fm_align_and_index(readers, desc, &Ob_Fm_Tt_oprops);
  let (mut Ob_Fm_Tf_oprop_aligns, mut Ob_Fm_Tf_oprop_indices) =
    get_Ob_Fm_align_and_index(readers, desc, &Ob_Fm_Tf_oprops);

  let (mut Oi_Fs_Tt_Mt_oprop_aligns, mut Oi_Fs_Tt_Mt_oprop_indices) =
    get_Oi_Fs_align_and_index(readers, desc, &Oi_Fs_Tt_Mt_oprops);
  let (mut Oi_Fs_Tf_Mt_oprop_aligns, mut Oi_Fs_Tf_Mt_oprop_indices) =
    get_Oi_Fs_align_and_index(readers, desc, &Oi_Fs_Tf_Mt_oprops);
  let (mut Oi_Fm_Tt_Mt_oprop_aligns, mut Oi_Fm_Tt_Mt_oprop_indices) =
    get_Oi_Fm_align_and_index(readers, desc, &Oi_Fm_Tt_Mt_oprops);
  let (mut Oi_Fm_Tf_Mt_oprop_aligns, mut Oi_Fm_Tf_Mt_oprop_indices) =
    get_Oi_Fm_align_and_index(readers, desc, &Oi_Fm_Tf_Mt_oprops);
  let (mut Oi_Fs_Tt_Mf_oprop_aligns, mut Oi_Fs_Tt_Mf_oprop_indices) =
    get_Oi_Fs_align_and_index(readers, desc, &Oi_Fs_Tt_Mf_oprops);
  let (mut Oi_Fs_Tf_Mf_oprop_aligns, mut Oi_Fs_Tf_Mf_oprop_indices) =
    get_Oi_Fs_align_and_index(readers, desc, &Oi_Fs_Tf_Mf_oprops);
  let (mut Oi_Fm_Tt_Mf_oprop_aligns, mut Oi_Fm_Tt_Mf_oprop_indices) =
    get_Oi_Fm_align_and_index(readers, desc, &Oi_Fm_Tt_Mf_oprops);
  let (mut Oi_Fm_Tf_Mf_oprop_aligns, mut Oi_Fm_Tf_Mf_oprop_indices) =
    get_Oi_Fm_align_and_index(readers, desc, &Oi_Fm_Tf_Mf_oprops);
  let mut external_subj = if let Subject::ExternalIDSubject(subj) = &class_plan.subject {
    Some((
      subj
        .real_id
        .0
        .path
        .get_initial_step(readers[subj.real_id.0.resource_id].as_ref()),
      build_align_func(&readers, desc, &subj.real_id.1).into_single(),
    ))
  } else {
    None
  };
  let n_Mt_dprops = Mt_dprops.len();
  let n_Mf_dprops = Mf_dprops.len();
  let n_Ob_Fs_Tt_oprops = Ob_Fs_Tt_oprops.len();
  let n_Ob_Fs_Tf_oprops = Ob_Fs_Tf_oprops.len();
  let n_Ob_Fm_Tt_oprops = Ob_Fm_Tt_oprops.len();
  let n_Ob_Fm_Tf_oprops = Ob_Fm_Tf_oprops.len();
  let n_Oi_Fs_Tt_Mt_oprops = Oi_Fs_Tt_Mt_oprops.len();
  let n_Oi_Fs_Tf_Mt_oprops = Oi_Fs_Tf_Mt_oprops.len();
  let n_Oi_Fm_Tt_Mt_oprops = Oi_Fm_Tt_Mt_oprops.len();
  let n_Oi_Fm_Tf_Mt_oprops = Oi_Fm_Tf_Mt_oprops.len();
  let n_Oi_Fs_Tt_Mf_oprops = Oi_Fs_Tt_Mf_oprops.len();
  let n_Oi_Fs_Tf_Mf_oprops = Oi_Fs_Tf_Mf_oprops.len();
  let n_Oi_Fm_Tt_Mf_oprops = Oi_Fm_Tt_Mf_oprops.len();
  let n_Oi_Fm_Tf_Mf_oprops = Oi_Fm_Tf_Mf_oprops.len();

  //  debug_assert!(
  //    max_Mt_dprops == 2 &&
  //    max_Mf_dprops == 3 &&
  //    max_Ob_Fs_Tt_oprops == 2 &&
  //    max_Ob_Fs_Tf_oprops == 0 &&
  //    max_Ob_Fm_Tt_oprops == 2 &&
  //    max_Ob_Fm_Tf_oprops == 0 &&
  //    max_Oi_Fs_Tt_Mt_oprops == 2 &&
  //    max_Oi_Fs_Tf_Mt_oprops == 0 &&
  //    max_Oi_Fm_Tt_Mt_oprops == 2 &&
  //    max_Oi_Fm_Tf_Mt_oprops == 0 &&
  //    max_Oi_Fs_Tt_Mf_oprops == 2 &&
  //    max_Oi_Fs_Tf_Mf_oprops == 0 &&
  //    max_Oi_Fm_Tt_Mf_oprops == 2 &&
  //    max_Oi_Fm_Tf_Mf_oprops == 0
  //  );
  // Note that unrolling dprops doesn't have much affect on the final performance.
  // unrolling the for loop doesn't have much effect.
  match &class_plan.subject {
    Subject::BlankSubject(subj) => {
      // reverse order
      //      recursive_num_seq_strict!(,
      //        4, n_Mf_dprops,
      //        0, n_Mt_dprops,
      //
      //        0, n_Oi_Fm_Tf_Mf_oprops,
      //        0, n_Oi_Fm_Tt_Mf_oprops,
      //        0, n_Oi_Fs_Tf_Mf_oprops,
      //        0, n_Oi_Fs_Tt_Mf_oprops,
      //        0, n_Oi_Fm_Tf_Mt_oprops,
      //        0, n_Oi_Fm_Tt_Mt_oprops,
      //        0, n_Oi_Fs_Tf_Mt_oprops,
      //        0, n_Oi_Fs_Tt_Mt_oprops,
      //        0, n_Ob_Fm_Tf_oprops,
      //        0, n_Ob_Fm_Tt_oprops,
      //        0, n_Ob_Fs_Tf_oprops,
      //        2, n_Ob_Fs_Tt_oprops
      //        ; class_optional_map,
      //        readers, cls_writer, class_plan,
      //        subj, blanksubject, external_subj,
      //
      //        Mt_dprop_aligns, Mt_dprop_indices,
      //        Mf_dprop_aligns, Mf_dprop_indices,
      //        Ob_Fs_Tt_oprops, Ob_Fs_Tt_oprop_aligns, Ob_Fs_Tt_oprop_indices,
      //        Ob_Fs_Tf_oprops, Ob_Fs_Tf_oprop_aligns, Ob_Fs_Tf_oprop_indices,
      //        Ob_Fm_Tt_oprops, Ob_Fm_Tt_oprop_aligns, Ob_Fm_Tt_oprop_indices,
      //        Ob_Fm_Tf_oprops, Ob_Fm_Tf_oprop_aligns, Ob_Fm_Tf_oprop_indices,
      //        Oi_Fs_Tt_Mt_oprops, Oi_Fs_Tt_Mt_oprop_aligns, Oi_Fs_Tt_Mt_oprop_indices,
      //        Oi_Fs_Tf_Mt_oprops, Oi_Fs_Tf_Mt_oprop_aligns, Oi_Fs_Tf_Mt_oprop_indices,
      //        Oi_Fm_Tt_Mt_oprops, Oi_Fm_Tt_Mt_oprop_aligns, Oi_Fm_Tt_Mt_oprop_indices,
      //        Oi_Fm_Tf_Mt_oprops, Oi_Fm_Tf_Mt_oprop_aligns, Oi_Fm_Tf_Mt_oprop_indices,
      //        Oi_Fs_Tt_Mf_oprops, Oi_Fs_Tt_Mf_oprop_aligns, Oi_Fs_Tt_Mf_oprop_indices,
      //        Oi_Fs_Tf_Mf_oprops, Oi_Fs_Tf_Mf_oprop_aligns, Oi_Fs_Tf_Mf_oprop_indices,
      //        Oi_Fm_Tt_Mf_oprops, Oi_Fm_Tt_Mf_oprop_aligns, Oi_Fm_Tt_Mf_oprop_indices,
      //        Oi_Fm_Tf_Mf_oprops, Oi_Fm_Tf_Mf_oprop_aligns, Oi_Fm_Tf_Mf_oprop_indices,
      //
      //        always_new_subject true, has_no_buffered_object_props true,
      //        no_missing_subj_values true
      //      );
    }
    Subject::InternalIDSubject(subj) => {
      //      recursive_num_seq_strict!(,
      //        4, n_Mf_dprops,
      //        0, n_Mt_dprops,
      //
      //        0, n_Oi_Fm_Tf_Mf_oprops,
      //        0, n_Oi_Fm_Tt_Mf_oprops,
      //        0, n_Oi_Fs_Tf_Mf_oprops,
      //        0, n_Oi_Fs_Tt_Mf_oprops,
      //        0, n_Oi_Fm_Tf_Mt_oprops,
      //        0, n_Oi_Fm_Tt_Mt_oprops,
      //        0, n_Oi_Fs_Tf_Mt_oprops,
      //        0, n_Oi_Fs_Tt_Mt_oprops,
      //        0, n_Ob_Fm_Tf_oprops,
      //        0, n_Ob_Fm_Tt_oprops,
      //        0, n_Ob_Fs_Tf_oprops,
      //        2, n_Ob_Fs_Tt_oprops
      //
      //        ; class_optional_map,
      //        readers, cls_writer, class_plan,
      //        subj, internalidsubject, external_subj,
      //
      //        Mt_dprop_aligns, Mt_dprop_indices,
      //        Mf_dprop_aligns, Mf_dprop_indices,
      //        Ob_Fs_Tt_oprops, Ob_Fs_Tt_oprop_aligns, Ob_Fs_Tt_oprop_indices,
      //        Ob_Fs_Tf_oprops, Ob_Fs_Tf_oprop_aligns, Ob_Fs_Tf_oprop_indices,
      //        Ob_Fm_Tt_oprops, Ob_Fm_Tt_oprop_aligns, Ob_Fm_Tt_oprop_indices,
      //        Ob_Fm_Tf_oprops, Ob_Fm_Tf_oprop_aligns, Ob_Fm_Tf_oprop_indices,
      //        Oi_Fs_Tt_Mt_oprops, Oi_Fs_Tt_Mt_oprop_aligns, Oi_Fs_Tt_Mt_oprop_indices,
      //        Oi_Fs_Tf_Mt_oprops, Oi_Fs_Tf_Mt_oprop_aligns, Oi_Fs_Tf_Mt_oprop_indices,
      //        Oi_Fm_Tt_Mt_oprops, Oi_Fm_Tt_Mt_oprop_aligns, Oi_Fm_Tt_Mt_oprop_indices,
      //        Oi_Fm_Tf_Mt_oprops, Oi_Fm_Tf_Mt_oprop_aligns, Oi_Fm_Tf_Mt_oprop_indices,
      //        Oi_Fs_Tt_Mf_oprops, Oi_Fs_Tt_Mf_oprop_aligns, Oi_Fs_Tt_Mf_oprop_indices,
      //        Oi_Fs_Tf_Mf_oprops, Oi_Fs_Tf_Mf_oprop_aligns, Oi_Fs_Tf_Mf_oprop_indices,
      //        Oi_Fm_Tt_Mf_oprops, Oi_Fm_Tt_Mf_oprop_aligns, Oi_Fm_Tt_Mf_oprop_indices,
      //        Oi_Fm_Tf_Mf_oprops, Oi_Fm_Tf_Mf_oprop_aligns, Oi_Fm_Tf_Mf_oprop_indices,
      //
      //        always_new_subject true, has_no_buffered_object_props true,
      //        no_missing_subj_values true
      //      );
    }
    Subject::ExternalIDSubject(subj) => {
      unimplemented!()
      //      if class_plan.buffered_object_props.len() == 0 {
      //        class_optional_map!(
      //          readers, cls_writer, class_plan,
      //          subj, externalidsubject, external_subj,
      //          always_new_subject true, has_no_buffered_object_props true
      //         );
      //      } else {
      //        class_optional_map!(
      //          readers, cls_writer, class_plan,
      //          subj, externalidsubject, external_subj,
      //          always_new_subject true
      //        );
      //      }
    }
  }
  return true;
}

fn split_dprops<'a0: 'a, 'a>(
  class_plan: &'a ClassMapPlan<'a0>,
) -> (Vec<&'a DataProp<'a0>>, Vec<&'a DataProp<'a0>>) {
  let mut Mt_dprops = vec![];
  let mut Mf_dprops = vec![];
  for dprop in class_plan.data_props.iter() {
    if dprop.missing_values.len() == 0 {
      Mf_dprops.push(dprop);
    } else {
      Mt_dprops.push(dprop);
    }
  }
  (Mt_dprops, Mf_dprops)
}

fn split_oprops<'a0: 'a, 'a>(
  class_plan: &'a ClassMapPlan<'a0>,
) -> (
  Vec<&'a BlankObject<'a0>>,
  Vec<&'a BlankObject<'a0>>,
  Vec<&'a BlankObject<'a0>>,
  Vec<&'a BlankObject<'a0>>,
  Vec<&'a IDObject<'a0>>,
  Vec<&'a IDObject<'a0>>,
  Vec<&'a IDObject<'a0>>,
  Vec<&'a IDObject<'a0>>,
  Vec<&'a IDObject<'a0>>,
  Vec<&'a IDObject<'a0>>,
  Vec<&'a IDObject<'a0>>,
  Vec<&'a IDObject<'a0>>,
) {
  let mut Ob_Fs_Tt_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Ob_Fs_Tf_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Ob_Fm_Tt_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Ob_Fm_Tf_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Oi_Fs_Tt_Mt_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Oi_Fs_Tf_Mt_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Oi_Fm_Tt_Mt_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Oi_Fm_Tf_Mt_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Oi_Fs_Tt_Mf_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Oi_Fs_Tf_Mf_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Oi_Fm_Tt_Mf_oprops = Vec::with_capacity(class_plan.object_props.len());
  let mut Oi_Fm_Tf_Mf_oprops = Vec::with_capacity(class_plan.object_props.len());
  for prop in class_plan.object_props.iter() {
    match prop {
      ObjectProp::BlankObject(oprop) => {
        // Ob (blank node)
        if oprop.alignments_cardinality.is_any2one() {
          // Fs
          if oprop.is_target_optional {
            // Tt
            Ob_Fs_Tt_oprops.push(oprop);
          } else {
            Ob_Fs_Tf_oprops.push(oprop);
          }
        } else {
          // Fm
          if oprop.is_target_optional {
            // Tt
            Ob_Fm_Tt_oprops.push(oprop);
          } else {
            Ob_Fm_Tf_oprops.push(oprop);
          }
        }
      }
      ObjectProp::IDObject(oprop) => {
        // Ob (blank node)
        if oprop.alignments_cardinality.is_any2one() {
          // Fs
          if oprop.is_target_optional {
            // Tt
            if oprop.missing_values.len() == 0 {
              Oi_Fs_Tt_Mf_oprops.push(oprop);
            } else {
              Oi_Fs_Tt_Mt_oprops.push(oprop);
            }
          } else {
            if oprop.missing_values.len() == 0 {
              Oi_Fs_Tf_Mf_oprops.push(oprop);
            } else {
              Oi_Fs_Tf_Mt_oprops.push(oprop);
            }
          }
        } else {
          // Fm
          if oprop.is_target_optional {
            // Tt
            if oprop.missing_values.len() == 0 {
              Oi_Fm_Tt_Mf_oprops.push(oprop);
            } else {
              Oi_Fm_Tt_Mt_oprops.push(oprop);
            }
          } else {
            if oprop.missing_values.len() == 0 {
              Oi_Fm_Tf_Mf_oprops.push(oprop);
            } else {
              Oi_Fm_Tf_Mt_oprops.push(oprop);
            }
          }
        }
      }
    }
  }
  (
    Ob_Fs_Tt_oprops,
    Ob_Fs_Tf_oprops,
    Ob_Fm_Tt_oprops,
    Ob_Fm_Tf_oprops,
    Oi_Fs_Tt_Mt_oprops,
    Oi_Fs_Tf_Mt_oprops,
    Oi_Fm_Tt_Mt_oprops,
    Oi_Fm_Tf_Mt_oprops,
    Oi_Fs_Tt_Mf_oprops,
    Oi_Fs_Tf_Mf_oprops,
    Oi_Fm_Tt_Mf_oprops,
    Oi_Fm_Tf_Mf_oprops,
  )
}

#[inline]
fn get_dalign_and_index<'a>(
  readers: &'a [Box<dyn RAReader + 'a>],
  desc: &Description,
  dprops: &Vec<&DataProp>,
) -> (Vec<Box<dyn SAlignmentFunc + 'a>>, Vec<Vec<Index>>) {
  let dprop_aligns = dprops
    .iter()
    .map(|a| build_align_func(&readers, desc, &a.alignments).into_single())
    .collect::<Vec<_>>();
  let dprop_indices = dprops
    .iter()
    .map(|p| {
      p.attribute
        .path
        .get_initial_step(readers[p.attribute.resource_id].as_ref())
    })
    .collect::<Vec<_>>();
  (dprop_aligns, dprop_indices)
}

#[inline]
fn get_Ob_Fs_align_and_index<'a>(
  readers: &'a [Box<dyn RAReader + 'a>],
  desc: &Description,
  oprops: &Vec<&BlankObject>,
) -> (Vec<Box<dyn SAlignmentFunc + 'a>>, Vec<Vec<Index>>) {
  let oprop_aligns = oprops
    .iter()
    .map(|a| build_align_func(&readers, desc, &a.alignments).into_single())
    .collect::<Vec<_>>();
  let oprop_indices = oprops
    .iter()
    .map(|p| {
      p.attribute
        .path
        .get_initial_step(readers[p.attribute.resource_id].as_ref())
    })
    .collect::<Vec<_>>();
  (oprop_aligns, oprop_indices)
}

#[inline]
fn get_Ob_Fm_align_and_index<'a>(
  readers: &'a [Box<dyn RAReader + 'a>],
  desc: &Description,
  oprops: &Vec<&BlankObject>,
) -> (Vec<Box<dyn MAlignmentFunc + 'a>>, Vec<Vec<Index>>) {
  let oprop_aligns = oprops
    .iter()
    .map(|a| build_align_func(&readers, desc, &a.alignments).into_multiple())
    .collect::<Vec<_>>();
  let oprop_indices = oprops
    .iter()
    .map(|p| {
      p.attribute
        .path
        .get_initial_step(readers[p.attribute.resource_id].as_ref())
    })
    .collect::<Vec<_>>();
  (oprop_aligns, oprop_indices)
}

#[inline]
fn get_Oi_Fs_align_and_index<'a>(
  readers: &'a [Box<dyn RAReader + 'a>],
  desc: &Description,
  oprops: &Vec<&IDObject>,
) -> (Vec<Box<dyn SAlignmentFunc + 'a>>, Vec<Vec<Index>>) {
  let oprop_aligns = oprops
    .iter()
    .map(|a| build_align_func(&readers, desc, &a.alignments).into_single())
    .collect::<Vec<_>>();
  let oprop_indices = oprops
    .iter()
    .map(|p| {
      p.attribute
        .path
        .get_initial_step(readers[p.attribute.resource_id].as_ref())
    })
    .collect::<Vec<_>>();
  (oprop_aligns, oprop_indices)
}

#[inline]
fn get_Oi_Fm_align_and_index<'a>(
  readers: &'a [Box<dyn RAReader + 'a>],
  desc: &Description,
  oprops: &Vec<&IDObject>,
) -> (Vec<Box<dyn MAlignmentFunc + 'a>>, Vec<Vec<Index>>) {
  let oprop_aligns = oprops
    .iter()
    .map(|a| build_align_func(&readers, desc, &a.alignments).into_multiple())
    .collect::<Vec<_>>();
  let oprop_indices = oprops
    .iter()
    .map(|p| {
      p.attribute
        .path
        .get_initial_step(readers[p.attribute.resource_id].as_ref())
    })
    .collect::<Vec<_>>();
  (oprop_aligns, oprop_indices)
}
