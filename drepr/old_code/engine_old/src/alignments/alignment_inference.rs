use crate::alignments::*;
use crate::models::{AlignmentFactory, BasedAlignedDim, DimAlignFactory, ChainAlignFactory, Representation};
use crate::readers::RAReader;
use fnv::FnvHashMap;
use petgraph::prelude::*;
use petgraph::visit::{depth_first_search, Control, DfsEvent};

pub struct BasicAlignmentInference<'a> {
  repr: &'a Representation,
  afuncs: Vec<Vec<Option<AlignmentFactory>>>,
}

impl<'a> BasicAlignmentInference<'a> {
  pub fn new(repr: &'a Representation) -> BasicAlignmentInference {
    let mut afuncs = vec![vec![None; repr.variables.len()]; repr.variables.len()];

    for a in &repr.alignments {
      match a {
        AlignmentFactory::DimAlign(da) => {
          afuncs[da.source][da.target] = Some(a.clone());
          afuncs[da.target][da.source] = Some(AlignmentFactory::DimAlign(da.swap()));
        }
        AlignmentFactory::ValueAlign(va) => {
          afuncs[va.source][va.target] = Some(a.clone());
          afuncs[va.target][va.source] = Some(AlignmentFactory::ValueAlign(va.swap()));
        }
        AlignmentFactory::IdenticalAlignment => panic!("Cannot have identical alignment"),
        AlignmentFactory::ChainAlign(_) => panic!("Cannot have chain alignment")
      }
    }

    for u in 0..repr.variables.len() {
      afuncs[u][u] = Some(AlignmentFactory::IdenticalAlignment)
    }

    let mut inference = BasicAlignmentInference { repr, afuncs };

    inference.inference();
    inference
  }

  /// find all variables that have single alignments to all variables in `vars`
  pub fn find_single_alignments(&self, vars: &[usize]) -> Vec<usize> {
    let mut primary_keys = vec![];
    for u in 0..self.repr.variables.len() {
      let mut u_is_pk = true;

      for &v in vars {
        if u == v {
          continue;
        }

        match &self.afuncs[u][v] {
          None => {
            u_is_pk = false;
            break;
          }
          Some(afunc) => {
            if !afunc.is_single(&self.repr.variables[u], &self.repr.variables[v]) {
              u_is_pk = false;
              break;
            }
          }
        }
      }

      if u_is_pk {
        primary_keys.push(u);
      }
    }

    return primary_keys;
  }

  /// find an alignment function between two variables
  pub fn find_alignment<'b, R: RAReader>(
    &self,
    ra_reader: &'b R,
    source: usize,
    target: usize,
  ) -> Option<AlignmentFunc<'b>> {
    match &self.afuncs[source][target] {
      None => None,
      Some(afunc) => Some(afunc.to_alignment_func(
        ra_reader,
        &self.repr.variables,
      )),
    }
  }

  /// Perform inference to find all possible alignment functions
  fn inference(&mut self) {
    let mut mg = GraphMap::<usize, (), Directed>::default();
    for vid in 0..self.repr.variables.len() {
      mg.add_node(vid);
    }

    for a in &self.repr.alignments {
      match a {
        AlignmentFactory::DimAlign(da) => {
          mg.add_edge(da.source, da.target, ());
          mg.add_edge(da.target, da.source, ());
        }
        AlignmentFactory::ValueAlign(va) => {
          mg.add_edge(va.source, va.target, ());
          mg.add_edge(va.target, va.source, ());
        }
        AlignmentFactory::IdenticalAlignment => panic!("Should not have identical alignment"),
        AlignmentFactory::ChainAlign(_) => panic!("Should not have chained alignment")
      }
    }

    // infer more mapping functions, trying to build a complete graph using DFS
    for u0 in 0..self.repr.variables.len() {
      let mut new_funcs: Vec<usize> = vec![];

      depth_first_search(&mg, Some(u0), |event| {
        if let DfsEvent::TreeEdge(u1, u2) = event {
          if u0 == u1 {
            return Control::Continue;
          }

          // try to infer mapping function between u0 and u2
          match self.infer_func(u0, u1, u2) {
            None => {
              // don't haven't find any
              return Control::Break(u2);
            }
            Some(afunc) => {
              new_funcs.push(u2);
              self.afuncs[u2][u0] = afunc.swap();
              self.afuncs[u0][u2] = Some(afunc);
            }
          }
        }

        Control::Continue
      });

      for ui in new_funcs {
        mg.add_edge(u0, ui, ());
      }
    }
  }

  /// Infer an alignment function: h: xid -> zid given f: xid -> yid and g: yid -> zid
  fn infer_func(&self, xid: usize, yid: usize, zid: usize) -> Option<AlignmentFactory> {
    let x = &self.repr.variables[xid];
    let y = &self.repr.variables[yid];
    let z = &self.repr.variables[zid];
    let f = self.afuncs[xid][yid].as_ref().unwrap();
    let g = self.afuncs[yid][zid].as_ref().unwrap();

    if f.is_dim_align() && g.is_dim_align() {
      if f.is_single(x, y) && g.is_single(y, z) {
        // handle the very basic case, we can do better than this
        let y2x: FnvHashMap<usize, usize> = f
          .as_dim_align()
          .aligned_dims
          .iter()
          .map(|x2y| (x2y.target_dim, x2y.source_dim))
          .collect();
        let mut x2z = vec![];

        for y2z in &g.as_dim_align().aligned_dims {
          x2z.push(BasedAlignedDim {
            source_dim: y2x[&y2z.source_dim],
            target_dim: y2z.target_dim,
          });
        }

        return Some(AlignmentFactory::DimAlign(DimAlignFactory {
          source: xid,
          target: yid,
          aligned_dims: x2z,
        }));
      }
    } else if f.is_value_align() && g.is_dim_align() {
      // fix me! hard code here
      if g.is_single(y, z) {
        return Some(AlignmentFactory::ChainAlign(ChainAlignFactory {
          immediate_target: yid,
          align0: Box::new(f.clone()),
          align1: Box::new(g.clone())
        }))
      }
    }

    return None;
  }
}
