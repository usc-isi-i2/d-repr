use crate::alignments::AlignmentFunc;
use crate::models::*;
use crate::executors::common::pseudo_id::ClassPseudoID;
use crate::models::Representation;
use crate::executors::v01::planning::topological_sorting::topological_sorting;
use crate::models::semantic_model::GraphNode;
use crate::alignments::BasicAlignmentInference;
use fnv::FnvHashMap;
use crate::readers::{RAReader, Value};

/// represent the plan to get primary key of a class
/// the primary key of a class can be: auto-generated (blank node)
/// or it is from another variable
#[derive(Debug)]
pub struct PrimaryKeyPlan<'a> {
  pub location: &'a Location,
  pub pseudo_id: ClassPseudoID,
  // some if the value of primary key is coming from another variable
  pub target: Option<(&'a Variable, AlignmentFunc<'a>)>,
  pub is_blank_node: bool,
}

#[derive(Debug)]
pub struct DataPropPlan<'a> {
  pub alignment: AlignmentFunc<'a>,
  pub predicate_id: usize,
  pub target_variable: &'a Variable,
  pub is_optional: bool
}

#[derive(Debug)]
pub struct LiteralPropPlan {
  pub predicate_id: usize,
  pub value: Value
}

#[derive(Debug)]
pub struct ObjectPropPlan<'a> {
  pub alignment: AlignmentFunc<'a>,
  pub predicate_id: usize,
  pub target_class: usize,
  pub target_pseudo_id: ClassPseudoID,
  pub target_variable: &'a Variable,
  pub is_optional: bool
}

#[derive(Debug)]
pub struct ClassPlan<'a> {
  pub class_id: usize,
  pub pk: PrimaryKeyPlan<'a>,
  pub literal_props: Vec<LiteralPropPlan>,
  pub data_props: Vec<DataPropPlan<'a>>,
  pub object_props: Vec<ObjectPropPlan<'a>>,
  pub buffered_object_props: Vec<ObjectPropPlan<'a>>
}

/// Create plans for mapping data
///
/// We create a topological sort of a semantic model, if the semantic model has cycle, we break
/// the cycle, and put the edges that cause the cycle into buffered vectors, and generate a topological sort
///
/// Then, we identify primary key of each class, and generate the plans as usual
pub fn create_mapping_plans<'a, R: RAReader>(ra_reader: &'a R, repr: &'a Representation) -> Vec<ClassPlan<'a>> {
  let reversed_topo_orders = topological_sorting(&repr.semantic_model);
  let mut class_plans = vec![];
  let inference = BasicAlignmentInference::new(repr);

  // find primary keys of each class
  let mut class2pk: FnvHashMap<usize, usize> = FnvHashMap::default();
  for &class_node in &reversed_topo_orders.topo_order {
    let data_nodes = repr.semantic_model.outgoing_edges[class_node]
      .iter()
      .filter(|e| match repr.semantic_model.nodes[repr.semantic_model.edges[**e].target] {
        GraphNode::DataNode(_) => true,
        _ => false
      })
      .map(|e| repr.semantic_model.nodes[repr.semantic_model.edges[*e].target].as_data_node().var_id)
      .collect::<Vec<usize>>();

    // identify primary keys, drepr:uri may not be the correct primary keys
    let primary_keys = inference.find_single_alignments(&data_nodes);
    if primary_keys.len() == 0 {
      panic!("There is no primary key of class: {}. User need to specify it explicitly", repr.semantic_model.nodes[class_node].get_label());
    }

    // TODO: heuristicly pick the best primary key, if we can pick the drepr:uri, what would be the best
    let primary_key = primary_keys[0];
    class2pk.insert(class_node, primary_key);
  }

  // generate plans
  for class_node in reversed_topo_orders.topo_order {
    let mut literal_props = vec![];
    let mut data_props = vec![];
    let mut object_props = vec![];
    let mut buffered_object_props = vec![];

    let mut drepr_uri_var_id = None;

    for &e in &repr.semantic_model.outgoing_edges[class_node] {
      let edge = &repr.semantic_model.edges[e];

      // TODO: move all special property into a global structure
      if edge.rel_label == "drepr:uri" {
        drepr_uri_var_id = Some(repr.semantic_model.nodes[edge.target].as_data_node().var_id);
        continue;
      }

      match &repr.semantic_model.nodes[edge.target] {
        GraphNode::ClassNode(n) => {
          let plan = ObjectPropPlan {
            alignment: inference.find_alignment(ra_reader, class2pk[&class_node], class2pk[&n.node_id]).expect(&format!("Need to has alignment between pk of two classes: {} - {}", class2pk[&class_node], class2pk[&n.node_id])),
            predicate_id: edge.edge_id,
            target_class: n.node_id,
            target_pseudo_id: ClassPseudoID { prefix: n.get_pseudo_prefix(), unbounded_dims: repr.variables[class2pk[&n.node_id]].location.get_unbounded_dims() },
            target_variable: &repr.variables[class2pk[&n.node_id]],
            is_optional: true
          };

          if reversed_topo_orders.removed_outgoing_edges[e] {
            buffered_object_props.push(plan);
          } else {
            object_props.push(plan);
          };
        },
        GraphNode::DataNode(n) => {
          data_props.push(DataPropPlan {
            alignment: inference.find_alignment(ra_reader, class2pk[&class_node], n.var_id).unwrap(),
            predicate_id: edge.edge_id,
            target_variable: &repr.variables[n.var_id],
            is_optional: true
          });
        },
        GraphNode::LiteralNode(n) => {
          literal_props.push(LiteralPropPlan {
            predicate_id: edge.edge_id,
            value: Value::Str(n.val.clone())
          });
        }
      }
    }

    class_plans.push(ClassPlan {
      class_id: class_node,
      pk: PrimaryKeyPlan {
        location: &repr.variables[class2pk[&class_node]].location,
        pseudo_id: ClassPseudoID {
          prefix: repr.semantic_model.nodes[class_node].as_class_node().get_pseudo_prefix(),
          unbounded_dims: repr.variables[class2pk[&class_node]].location.get_unbounded_dims()
        },
        target: match drepr_uri_var_id {
          None => None,
          Some(var_id) => {
            Some((&repr.variables[var_id], inference.find_alignment(ra_reader, class2pk[&class_node], var_id).unwrap()))
          }
        },
        is_blank_node: drepr_uri_var_id.is_none()
      },
      literal_props,
      data_props,
      object_props,
      buffered_object_props
    });
  }

  class_plans
}