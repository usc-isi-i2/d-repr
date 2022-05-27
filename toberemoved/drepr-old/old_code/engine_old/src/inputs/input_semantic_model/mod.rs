use hashbrown::HashMap;
use serde::Deserialize;

use crate::models::*;
use crate::utils::rdf;

use self::data_nodes::*;
use self::literal_nodes::*;
use self::relations::InputRelation;

mod data_nodes;
mod literal_nodes;
mod relations;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InputSemanticModel {
  #[serde(default)]
  data_nodes: std::collections::HashMap<String, InputDataNode>,
  #[serde(default)]
  literal_nodes: Vec<InputLiteralNode>,
  #[serde(default)]
  relations: Vec<InputRelation>,
  #[serde(default)]
  prefixes: std::collections::HashMap<String, String>,
}

impl InputSemanticModel {
  pub fn into_semantic_model(self, var_name2id: &HashMap<&str, usize>) -> SemanticModel {
    let mut idmap: HashMap<String, usize> = HashMap::default();
    let mut nodes = vec![];
    let mut edges = vec![];

    for (cid, cname) in self.data_nodes
      .values()
      .map(|v| (&v.class_id, &v.class_name))
      .chain(self.literal_nodes
        .iter()
        .map(|v| (&v.class_id, &v.class_name)))
    {
      if idmap.get(cid).is_none() {
        idmap.insert(cid.clone(), idmap.len());

        let (rel_label, abs_label) = InputSemanticModel::get_rel_abs_label(&self.prefixes, cname);
        nodes.push(GraphNode::ClassNode(ClassNode {
          node_id: idmap[cid.as_str()],
          rel_label,
          abs_label
        }));
      }
    }

    for (var_name, c) in self.data_nodes.into_iter() {
      idmap.insert(var_name.clone(), idmap.len());
      let (rel_label, abs_label) = InputSemanticModel::get_rel_abs_label(&self.prefixes, &c.predicate);
      edges.push(Edge {
        edge_id: edges.len(),
        source: *idmap.get(&c.class_id).unwrap(),
        target: *idmap.get(&var_name).unwrap(),
        rel_label,
        abs_label
      });
      nodes.push(GraphNode::DataNode(DataNode {
        node_id: idmap.len() - 1,
        var_id: var_name2id[var_name.as_str()],
        var_name,
        data_type: c.data_type,
      }));
    }

    let mut n_literal = 0;
    for c in self.literal_nodes {
      let nid = format!("--&^%!&@:{}", n_literal);
      n_literal += 1;
      idmap.insert(nid.clone(), idmap.len());
      nodes.push(GraphNode::LiteralNode(LiteralNode {
        node_id: idmap[&nid],
        val: c.data.clone(),
        data_type: c.data_type,
      }));

      let (rel_label, abs_label) = InputSemanticModel::get_rel_abs_label(&self.prefixes, &c.predicate);
      edges.push(Edge {
        edge_id: edges.len(),
        source: *idmap.get(&c.class_id).unwrap(),
        target: *idmap.get(&nid).unwrap(),
        rel_label,
        abs_label
      });
    }

    for c in self.relations {
      let (rel_label, abs_label) = InputSemanticModel::get_rel_abs_label(&self.prefixes, &c.predicate);
      edges.push(Edge {
        edge_id: edges.len(),
        source: *idmap.get(&c.source_id).unwrap(),
        target: *idmap.get(&c.target_id).unwrap(),
        rel_label,
        abs_label
      })
    }

    SemanticModel::new(
      nodes,
      edges, self.prefixes.into_iter().collect::<Vec<(_, _)>>())
  }

  pub fn is_empty(&self) -> bool {
    self.data_nodes.len() == 0 && self.relations.len() == 0 && self.literal_nodes.len() == 0
  }

  fn get_rel_abs_label(prefixes: &std::collections::HashMap<String, String>, uri: &str) -> (String, Option<String>) {
    if rdf::is_absolute_uri(uri) {
      (uri.to_string(), None)
    } else {
      let (ns, reluri) = rdf::split_prefixed_uri(uri);
      (uri.to_string(), Some(format!("{}{}", prefixes[ns], reluri)))
    }
  }
}
