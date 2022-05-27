use crate::lang::semantic_model::SemanticModel;

#[derive(Debug, Clone)]
pub struct ReversedTopoSortResult {
  pub topo_order: Vec<usize>,
  pub removed_outgoing_edges: Vec<bool>,
}

/// suppose we have a semantic model, that may contains circle, our task is to
/// generate a topological order from the directed cyclic graph, and a set of edges
/// involved in the cycles that are removed in order to generate the topo-order
pub fn topological_sorting(sm: &SemanticModel) -> ReversedTopoSortResult {
  let mut visited_nodes = vec![false; sm.nodes.len()];
  let mut tmp_visited_nodes = vec![false; sm.nodes.len()];
  let mut removed_outgoing_edges = vec![false; sm.edges.len()];

  loop {
    // pick a node from the remaining nodes and do reverse dfs. if we found a cycle, then we break the cycle and repeat the process
    // until we no cycle left
    let mut random_start_node = 0;
    let mut has_unvisited_node = false;
    for (nid, &is_visited) in visited_nodes.iter().enumerate() {
      if !is_visited && sm.nodes[nid].is_class_node() {
        random_start_node = nid;
        has_unvisited_node = true;
        break;
      }
    }

    if !has_unvisited_node {
      // we don't have any remaining nodes, so we finish!
      break;
    }

    // preparing the data
    for i in 0..tmp_visited_nodes.len() {
      tmp_visited_nodes[i] = false;
    }

    // loop until it breaks all cycles
    while dfs_breaking_cycle(
      sm,
      random_start_node,
      &mut tmp_visited_nodes,
      &mut removed_outgoing_edges,
    ) {
      // reset the tmp visited nodes
      for i in 0..tmp_visited_nodes.len() {
        tmp_visited_nodes[i] = false;
      }
    }

    // mark visited nodes
    for i in 0..tmp_visited_nodes.len() {
      if tmp_visited_nodes[i] {
        visited_nodes[i] = true;
      }
    }
  }

  // now we get acyclic graph, determine the topo-order
  let mut reversed_topo_order = vec![];
  for i in 0..visited_nodes.len() {
    visited_nodes[i] = false;
    tmp_visited_nodes[i] = false;
  }

  for nid in 0..visited_nodes.len() {
    if !visited_nodes[nid] && sm.nodes[nid].is_class_node() {
      dfs_reverse_topo_sort(
        sm,
        &mut reversed_topo_order,
        nid,
        &mut visited_nodes,
        &mut tmp_visited_nodes,
        &removed_outgoing_edges,
      );
    }
  }

  return ReversedTopoSortResult {
    topo_order: reversed_topo_order,
    removed_outgoing_edges,
  };
}

/// Generate a topological order of class nodes in the semantic model. The graph must be acyclic
/// before using this function
///
/// Based on DFS algorithm in here: https://en.wikipedia.org/wiki/Topological_sorting
fn dfs_reverse_topo_sort(
  sm: &SemanticModel,
  topo_order: &mut Vec<usize>,
  node: usize,
  visited_nodes: &mut [bool],
  tmp_visited_nodes: &mut [bool],
  removed_outgoing_edges: &[bool],
) {
  if visited_nodes[node] {
    return;
  }

  if tmp_visited_nodes[node] {
    panic!("The graph has cycle!");
  }
  tmp_visited_nodes[node] = true;

  for &e in &sm.outgoing_edges[node] {
    if !removed_outgoing_edges[e] && sm.nodes[sm.edges[e].target].is_class_node() {
      dfs_reverse_topo_sort(
        sm,
        topo_order,
        sm.edges[e].target,
        visited_nodes,
        tmp_visited_nodes,
        removed_outgoing_edges,
      );
    }
  }

  tmp_visited_nodes[node] = false;
  visited_nodes[node] = true;
  topo_order.push(node);
}

/// Try to break cycles using invert DFS. It returns true when break one cycle, and it terminates
/// immediately. Thus, requires you to run this function many times until it return false
fn dfs_breaking_cycle(
  sm: &SemanticModel,
  node: usize,
  visited_nodes: &mut [bool],
  removed_outgoing_edges: &mut [bool],
) -> bool {
  visited_nodes[node] = true;

  for &e in &sm.incoming_edges[node] {
    if !removed_outgoing_edges[e] {
      if visited_nodes[sm.edges[e].source] {
        // this node is visited, and it is visited by traveling through `e`, we can drop `e` and move on
        removed_outgoing_edges[e] = true;
        return true;
      }

      if dfs_breaking_cycle(
        sm,
        sm.edges[e].source,
        visited_nodes,
        removed_outgoing_edges,
      ) {
        return true;
      }
    }
  }

  return false;
}
