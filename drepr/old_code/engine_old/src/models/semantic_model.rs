use crate::models::variable::Variable;

#[derive(Debug)]
pub struct ClassNode {
  pub node_id: usize,
  pub rel_label: String,
  // none if this is blank node
  pub abs_label: Option<String>,
}

#[derive(Debug)]
pub struct DataNode {
  pub node_id: usize,
  pub var_id: usize,
  pub var_name: String,
  pub data_type: Option<String>,
}

#[derive(Debug)]
pub struct LiteralNode {
  pub node_id: usize,
  pub val: String,
  pub data_type: Option<String>,
}

#[derive(Debug)]
pub enum GraphNode {
  ClassNode(ClassNode),
  DataNode(DataNode),
  LiteralNode(LiteralNode),
}

#[derive(Debug)]
pub struct Edge {
  pub edge_id: usize,
  pub source: usize,
  pub target: usize,
  pub rel_label: String,
  pub abs_label: Option<String>,
}

#[derive(Default, Debug)]
pub struct SemanticModel {
  // all class nodes must be before data nodes and literal nodes in the array
  pub nodes: Vec<GraphNode>,
  pub edges: Vec<Edge>,
  pub incoming_edges: Vec<Vec<usize>>,
  pub outgoing_edges: Vec<Vec<usize>>,
  pub prefixes: Vec<(String, String)>
}

impl SemanticModel {
  /// all class nodes must be before data nodes and literal nodes in the array
  pub fn new(nodes: Vec<GraphNode>, edges: Vec<Edge>, prefixes: Vec<(String, String)>) -> SemanticModel {
    let mut incoming_edges: Vec<Vec<usize>> = vec![vec![]; nodes.len()];
    let mut outgoing_edges: Vec<Vec<usize>> = vec![vec![]; nodes.len()];

    for (eid, edge) in edges.iter().enumerate() {
      incoming_edges[edge.target].push(eid);
      outgoing_edges[edge.source].push(eid);
    }

    SemanticModel {
      nodes,
      edges,
      incoming_edges,
      outgoing_edges,
      prefixes
    }
  }

  pub fn from_variables(variables: &[Variable]) -> SemanticModel {
    let mut nodes = vec![GraphNode::ClassNode(ClassNode {
      node_id: 0,
      rel_label: "Record".to_string(),
      abs_label: None,
    })];
    let mut edges = vec![];

    for (var_id, var) in variables.iter().enumerate() {
      let stype = DataNode {
        node_id: var_id + 1,
        var_id,
        var_name: var.name.clone(),
        data_type: Some("xsd:string".to_string()),
      };

      edges.push(Edge {
        edge_id: edges.len(),
        source: 0,
        target: nodes.len(),
        rel_label: var.name.clone(),
        abs_label: None,
      });
      nodes.push(GraphNode::DataNode(stype));
    }

    SemanticModel::new(nodes, edges, Default::default())
  }
}

impl GraphNode {
  pub fn is_class_node(&self) -> bool {
    match self {
      GraphNode::ClassNode(_) => true,
      _ => false,
    }
  }

  pub fn as_class_node(&self) -> &ClassNode {
    match self {
      GraphNode::ClassNode(x) => x,
      _ => panic!("Cannot cast non class node to class node"),
    }
  }

  pub fn as_data_node(&self) -> &DataNode {
    match self {
      GraphNode::DataNode(x) => x,
      _ => panic!("Cannot convert non data node to data node"),
    }
  }

  pub fn get_label(&self) -> &str {
    match self {
      GraphNode::ClassNode(x) => &x.rel_label,
      GraphNode::DataNode(x) => &x.var_name,
      GraphNode::LiteralNode(_) => "literal",
    }
  }
}

impl ClassNode {
  /// Compute a pseudo prefix from the label of the class node, so that it complies
  /// with standard turtle blank node
  pub fn get_pseudo_prefix(&self) -> String {
    let short_lbl = self.rel_label
      .rsplitn(2, "/").next().unwrap()
      .rsplitn(2, ":").next().unwrap();

    format!("{}{}", short_lbl, self.node_id)
  }


  pub fn is_blank_node(&self, sm: &SemanticModel) -> bool {
    for &e in &sm.outgoing_edges[self.node_id] {
      if sm.edges[e].rel_label == "drepr:uri" {
        return false;
      }
    }
    return true;
  }
}