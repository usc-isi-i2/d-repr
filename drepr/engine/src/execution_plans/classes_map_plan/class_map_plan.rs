use hashbrown::{HashSet};
use serde::{Deserialize, Serialize};
use readers::is_enum_type_impl;

use crate::alignments::inference::AlignmentInference;
use crate::execution_plans::classes_map_plan::data_prop::DataProp;
use crate::execution_plans::classes_map_plan::literal_prop::LiteralProp;
use crate::execution_plans::classes_map_plan::object_prop::{BlankObject, IDObject, ObjectProp};
use crate::execution_plans::classes_map_plan::subject::{BlankSubject, ExternalIDSubject, InternalIDSubject, Subject};
use crate::execution_plans::pseudo_id::ClassPseudoID;
use crate::lang::{Description, DREPR_URI, GraphNode};
use crate::writers::stream_writer::OutputFormat;

#[cfg(feature = "enable-exec-macro-cls-map")]
use crate::executors::classes_map::specific_algo::specific_class_map::analyze_specific_algo_strategy;

#[derive(Serialize, Debug)]
pub struct ClassMapPlan<'a> {
  pub class_id: usize,
  pub subject: Subject<'a>,
  pub data_props: Vec<DataProp<'a>>,
  pub literal_props: Vec<LiteralProp>,
  pub object_props: Vec<ObjectProp<'a>>,
  pub buffered_object_props: Vec<ObjectProp<'a>>,
  pub exec_strategy: ClassMapExecStrategy,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ClassMapExecStrategy {
  Generic,
  Macro(String)
}

impl<'a> ClassMapPlan<'a> {
  pub fn new(desc: &'a Description, _output_format: &OutputFormat, class_id: usize, class2subj: &[usize], inference: &AlignmentInference, edges_optional: &[bool], removed_edges: &[bool]) -> ClassMapPlan<'a> {
    let subj = class2subj[class_id];
    let uri_dnode = match desc.semantic_model.outgoing_edges[class_id].iter()
      .find(|eid| desc.semantic_model.edges[**eid].rel_label == DREPR_URI) {
      None => None,
      Some(eid) => Some(desc.semantic_model.nodes[desc.semantic_model.edges[*eid].target].as_data_node())
    };
    
    // generate other properties
    let mut literal_props = vec![];
    let mut data_props = vec![];
    let mut object_props = vec![];
    let mut buffered_object_props = vec![];
    
    for &eid in &desc.semantic_model.outgoing_edges[class_id] {
      match desc.semantic_model.get_target(eid) {
        GraphNode::DataNode(n) => {
          let attribute = &desc.attributes[n.attr_id];
          let edge = desc.semantic_model.get_edge(class_id, n.node_id).unwrap();
          
          if edge.rel_label != DREPR_URI {
            data_props.push(DataProp {
              alignments: inference.get_alignments(subj, n.attr_id),
              predicate_id: edge.edge_id,
              attribute,
              is_optional: edges_optional[edge.edge_id],
              missing_values: attribute.missing_values
                .iter()
                .map(|v| v.clone())
                .collect::<HashSet<_>>(),
            });
          }
        }
        GraphNode::LiteralNode(n) => {
          literal_props.push(LiteralProp {
            predicate_id: eid,
            value: n.val.clone(),
          });
        }
        GraphNode::ClassNode(n) => {
          let attribute = &desc.attributes[class2subj[n.node_id]];
          let predicate_id = eid;
          // a class node is optional if all of its properties are optional
          let is_target_optional = desc.semantic_model.outgoing_edges[n.node_id]
            .iter().all(|&eid| edges_optional[eid]);
          let alignments = inference.get_alignments(subj, attribute.id);

          let prop = if n.is_blank_node(&desc.semantic_model) {
            ObjectProp::BlankObject(BlankObject {
              attribute,
              alignments_cardinality: inference.estimate_cardinality(&alignments),
              alignments,
              pseudo_id: ClassPseudoID::new(format!("_:{}", n.get_pseudo_prefix()), attribute.path.get_nary_steps()),
              predicate_id,
              class_id,
              is_optional: edges_optional[predicate_id],
              is_target_optional,
            })
          } else {
            ObjectProp::IDObject(IDObject {
              attribute,
              alignments_cardinality: inference.estimate_cardinality(&alignments),
              alignments,
              pseudo_id: ClassPseudoID::new(format!("_:{}", n.get_pseudo_prefix()), attribute.path.get_nary_steps()),
              predicate_id,
              class_id,
              is_optional: edges_optional[predicate_id],
              is_target_optional,
              missing_values: attribute.missing_values
                .iter()
                .map(|v| v.clone())
                .collect::<HashSet<_>>(),
            })
          };

          if removed_edges[predicate_id] {
            buffered_object_props.push(prop);
          } else {
            object_props.push(prop);
          }
        }
      }
    }
    
    let subj_attr = &desc.attributes[subj];
    let subj_pseudo_id = ClassPseudoID::new(
      format!("_:{}", desc.semantic_model.nodes[class_id].as_class_node().get_pseudo_prefix()),
      desc.attributes[subj].path.get_nary_steps());
    let subject = match uri_dnode {
      None => {
        Subject::BlankSubject(BlankSubject {
          attr: subj_attr,
          pseudo_id: subj_pseudo_id,
        })
      }
      Some(uri_dnode) => {
        // get missing values from the real subjects
        let missing_values = desc.attributes[subj].missing_values.iter()
          .map(|v| v.clone())
          .collect::<HashSet<_>>();

        if uri_dnode.attr_id == subj {
          Subject::InternalIDSubject(InternalIDSubject {
            attr: subj_attr,
            pseudo_id: subj_pseudo_id,
            is_optional: edges_optional[desc.semantic_model.get_edge(class_id, uri_dnode.node_id).unwrap().edge_id],
            missing_values,
          })
        } else {
          Subject::ExternalIDSubject(ExternalIDSubject {
            attr: subj_attr,
            pseudo_id: subj_pseudo_id,
            real_id: (&desc.attributes[uri_dnode.attr_id], inference.get_alignments(subj, uri_dnode.attr_id)),
            is_optional: edges_optional[desc.semantic_model.get_edge(class_id, uri_dnode.attr_id).unwrap().edge_id],
            missing_values,
          })
        }
      }
    };
    
    #[allow(unused_mut)]
    let mut plan = ClassMapPlan {
      class_id,
      subject,
      data_props,
      literal_props,
      object_props,
      buffered_object_props,
      exec_strategy: ClassMapExecStrategy::Generic
    };
    
    #[cfg(feature = "enable-exec-macro-cls-map")]
    {
      if let Some(explanation) = analyze_specific_algo_strategy(&plan) {
        plan.exec_strategy = ClassMapExecStrategy::Macro(explanation)
      }
    }
    
    plan
  }
  
  /// Find the subject of the class
  pub fn find_subject(desc: &Description, class_id: usize, inference: &AlignmentInference) -> usize {
    // get data nodes, attributes, and the attribute that contains URIs of the class
    let mut data_nodes = vec![];
    let mut attrs = vec![];
    let mut uri_attr = None;
    
    for &eid in &desc.semantic_model.outgoing_edges[class_id] {
      let target = desc.semantic_model.edges[eid].get_target(&desc.semantic_model);
      
      if target.is_data_node() {
        let n = target.as_data_node();
        data_nodes.push(n);
        attrs.push(n.attr_id);
        
        if desc.semantic_model.edges[eid].rel_label == DREPR_URI {
          uri_attr = Some(n.attr_id);
        }
      }
    }
    
    // if the subject attribute is provided, then, we will use it
    let mut subjs = data_nodes
      .iter()
      .filter(|&n| {
        desc.semantic_model
          .get_edge(class_id, n.node_id).unwrap()
          .is_subject
      })
      .map(|n| n.attr_id).collect::<Vec<_>>();
    
    if subjs.len() == 0 {
      // invoke the inference to find the subject attribute
      subjs = inference.infer_subject(&attrs);
    }
    
    if subjs.len() == 0 {
      panic!("There is no subject attribute of class: {}. Users need to specify it explicitly", desc.semantic_model.nodes[class_id].as_class_node().rel_label);
    }
    
    ClassMapPlan::select_subject(desc, class_id, &subjs, &attrs, &uri_attr)
  }
  
  /// Select the best subject from a list of possible subjects. In the current approach, we pick
  /// the attribute that is associated with `drepr:uri` predicate.
  pub fn select_subject(_desc: &Description, _class_id: usize, subjs: &[usize], _attrs: &[usize], uri_attr: &Option<usize>) -> usize {
    if let Some(aid) = uri_attr {
      for &subj in subjs {
        if subj == *aid {
          return subj;
        }
      }
    }

    subjs[0]
  }
  
  pub fn is_optional(&self) -> bool {
    self.subject.is_optional() && self.data_props.iter().all(|p| p.is_optional) && self.object_props.iter().chain(self.buffered_object_props.iter()).all(|p| p.is_optional())
  }
}

impl ClassMapExecStrategy {
  is_enum_type_impl!(ClassMapExecStrategy::is_generic(Generic));
  is_enum_type_impl!(ClassMapExecStrategy::is_macro(Macro(_)));
}