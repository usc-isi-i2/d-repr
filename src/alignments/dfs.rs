use petgraph::visit::{GraphRef, IntoNeighbors, Visitable, VisitMap};
use fnv::FnvHashSet;
use std::hash::Hash;

/// THE FOLLOWING CODE IS COPIED FROM: https://docs.rs/petgraph/0.4.13/src/petgraph/visit/traversal.rs.html#38-43
/// so that DFS also yield the parent node
///
/// Visit nodes of a graph in a depth-first-search (DFS) emitting nodes in
/// preorder (when they are first discovered).
///
/// The traversal starts at a given node and only traverses nodes reachable
/// from it.
///
/// `Dfs` is not recursive.
///
/// `Dfs` does not itself borrow the graph, and because of this you can run
/// a traversal over a graph while still retaining mutable access to it, if you
/// use it like the following example:
///
/// ```
/// use petgraph::Graph;
/// use petgraph::visit::Dfs;
///
/// let mut graph = Graph::<_,()>::new();
/// let a = graph.add_node(0);
///
/// let mut dfs = Dfs::new(&graph, a);
/// while let Some(nx) = dfs.next(&graph) {
///     // we can access `graph` mutably here still
///     graph[nx] += 1;
/// }
///
/// assert_eq!(graph[a], 1);
/// ```
///
/// **Note:** The algorithm may not behave correctly if nodes are removed
/// during iteration. It may not necessarily visit added nodes or edges.
#[derive(Clone, Debug)]
pub struct CustomedDfs<N, VM> {
  /// The stack of nodes to visit
  pub stack: Vec<(N, N)>,
  /// The map of discovered nodes
  pub discovered: VM,
}

impl<N, VM> CustomedDfs<N, VM>
  where N: Copy + PartialEq + Eq + Hash,
        VM: VisitMap<N>,
{
  /// Create a new **Dfs**, using the graph's visitor map, and put **start**
  /// in the stack of nodes to visit.
  pub fn new<G>(graph: G, start: N) -> Self
    where G: GraphRef + Visitable<NodeId=N, Map=VM>
  {
    let mut dfs = CustomedDfs::empty(graph);
    dfs.move_to(start);
    dfs
  }

  /// Create a new **Dfs** using the graph's visitor map, and no stack.
  pub fn empty<G>(graph: G) -> Self
    where G: GraphRef + Visitable<NodeId=N, Map=VM>
  {
    CustomedDfs {
      stack: Vec::new(),
      discovered: graph.visit_map(),
    }
  }

  /// Keep the discovered map, but clear the visit stack and restart
  /// the dfs from a particular node.
  pub fn move_to(&mut self, start: N)
  {
    self.discovered.visit(start);
    self.stack.clear();
    self.stack.push((start, start));
  }

  /// Return the next node in the dfs, or **None** if the traversal is done.
  pub fn next<G>(&mut self, graph: G, revisit: &FnvHashSet<N>) -> Option<(N, N)>
    where G: IntoNeighbors<NodeId=N>,
  {
    if let Some((parent_node, node)) = self.stack.pop() {
      for succ in graph.neighbors(node) {
        if self.discovered.visit(succ) || revisit.contains(&succ) {
          self.stack.push((node, succ));
        }
      }
      return Some((parent_node, node));
    }
    None
  }


}
