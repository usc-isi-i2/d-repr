use serde::Serialize;

use crate::alignments::inference::AlignmentInference;
use crate::execution_plans::topological_sorting::topological_sorting;
use crate::lang::Description;
use crate::writers::stream_writer::{WriteMode, OutputFormat};

pub use self::class_map_plan::ClassMapPlan;
use self::read_plan::ReadPlan;
use self::subject::Subject;
use self::write_plan::WritePlan;
use crate::execution_plans::classes_map_plan::object_prop::ObjectProp;

pub mod read_plan;
pub mod write_plan;
pub mod class_map_plan;
pub mod subject;
pub mod data_prop;
pub mod literal_prop;
pub mod object_prop;

#[derive(Serialize, Debug)]
pub struct ClassesMapExecutionPlan<'a> {
  // one per resource
  pub read_plans: Vec<ReadPlan>,
  // a write plan may be just one single write or multiple writer for multiple classes
  pub write_plan: WritePlan,
  pub class_map_plans: Vec<ClassMapPlan<'a>>,
}

impl<'a> ClassesMapExecutionPlan<'a> {
  /// Create plans for mapping data
  ///
  /// We create a topological sort of a semantic model, if the semantic model has cycle, we break
  /// the cycle, and put the edges that cause the cycle into buffered vectors, and generate a
  /// topological sort
  ///
  /// Then, we identify subject of each class, and generate the plans as normal
  pub fn new(desc: &'a Description, output_format: &OutputFormat, edges_optional: &[bool]) -> ClassesMapExecutionPlan<'a> {
    let reversed_topo_orders = topological_sorting(&desc.semantic_model);
    let n_class_nodes = desc.semantic_model.get_n_class_nodes();
    let inference = AlignmentInference::new(desc);
    let mut class_map_plans = Vec::with_capacity(n_class_nodes);
    
    // find subject attribute of each class
    let mut class2subj: Vec<usize> = Vec::with_capacity(n_class_nodes);
    for class_id in 0..n_class_nodes {
      class2subj.push(ClassMapPlan::find_subject(desc, class_id, &inference));
    }
    
    // generate plans
    for class_id in reversed_topo_orders.topo_order {
      class_map_plans.push(ClassMapPlan::new(desc, output_format, class_id, &class2subj, &inference, &edges_optional, &reversed_topo_orders.removed_outgoing_edges));
    }
    
    // determine the writing strategy
    let mut class_write_modes = vec![WriteMode::Tt_Ut_Sb_Ob; n_class_nodes];
    for cls_plan in &class_map_plans {
      let obj_blank_or_uri_iter = cls_plan.object_props
        .iter().chain(cls_plan.buffered_object_props.iter())
        .map(|o| {
          match o {
            ObjectProp::BlankObject(_) => Some(true),
            ObjectProp::IDObject(v) => if v.missing_values.len() > 0 && v.is_optional {
              // uri is missing sometime so we have to use blank node
              None
            } else {
              // always uri
              Some(false)
            },
          }
        })
        .collect::<Vec<_>>();
      
      let obj_blank_or_uri = if obj_blank_or_uri_iter.iter().all(|v| v == &Some(true)) {
        Some(true)
      } else if obj_blank_or_uri_iter.iter().all(|v| v == &Some(false)) {
        Some(false)
      } else {
        None
      };
      
      let write_mode = WriteMode::create(
        !cls_plan.is_optional(), cls_plan.subject.is_unique(),
        match &cls_plan.subject {
            Subject::BlankSubject(_) => Some(true),
            Subject::InternalIDSubject(s) => if s.missing_values.len() > 0 && s.is_optional {
              // uri is missing sometime so we have to use blank node
              None
            } else {
              // always uri
              Some(false)
            },
            Subject::ExternalIDSubject(s) => if s.missing_values.len() > 0 && s.is_optional {
              // uri is missing sometime so we have to use blank node
              None
            } else {
              // always uri
              Some(false)
            },
        }, obj_blank_or_uri);
      
      class_write_modes[cls_plan.class_id] = write_mode;
    }
    
    ClassesMapExecutionPlan {
      read_plans: vec![ReadPlan::SeqRead; desc.resources.len()],
      write_plan: WritePlan::SingleWriter2File {
        class_write_modes
      },
      class_map_plans,
    }
  }
}