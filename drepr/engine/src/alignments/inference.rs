use hashbrown::{HashMap, HashSet};
use petgraph::prelude::*;

use crate::lang::{AlignedDim, Alignment, Cardinality, Description, RangeAlignment};

use super::dfs::CustomedDfs;
use fnv::FnvHashSet;

pub struct AlignmentInference<'a> {
  desc: &'a Description,
  aligns: Vec<Vec<Vec<Alignment>>>,
}

impl<'a> AlignmentInference<'a> {
  pub fn new(desc: &'a Description) -> AlignmentInference {
    let mut aligns = vec![vec![Vec::with_capacity(2); desc.attributes.len()]; desc.attributes.len()];
    
    for a in &desc.alignments {
      match a {
        Alignment::RangeAlign(da) => {
          aligns[da.source][da.target] = vec![a.clone()];
          aligns[da.target][da.source] = vec![Alignment::RangeAlign(da.swap())];
        }
        Alignment::ValueAlign(va) => {
          aligns[va.source][va.target] = vec![a.clone()];
          aligns[va.target][va.source] = vec![Alignment::ValueAlign(va.swap())];
        }
        Alignment::IdenticalAlign => unreachable!()
      }
    }
    
    for u in 0..desc.attributes.len() {
      aligns[u][u] = vec![Alignment::IdenticalAlign];
    }
    
    let mut inference = AlignmentInference { desc, aligns };
    
    inference.inference();
    inference
  }
  
  /// get an alignment function between two attributes
  #[inline]
  pub fn get_alignments(&self, source: usize, target: usize) -> Vec<Alignment> {
    self.aligns[source][target].clone()
  }
  
  /// Find all attributes that can be the subject of these attributes
  ///
  /// The subject must has one to one mapping
  pub fn infer_subject(&self, attrs: &[usize]) -> Vec<usize> {
    let mut subjs = vec![];
    
    // if one or these attributes has *-to-one, then that is the subjects
    for &a in attrs {
      let mut is_subj = true;
      
      for &ai in attrs {
        if self.aligns[a][ai].len() == 0 {
          // no alignment
          is_subj = false;
          break;
        }
        
        let cardin = self.estimate_cardinality(&self.aligns[a][ai]);
        match cardin {
          Cardinality::M2M => {
            is_subj = false;
            break;
          }
          Cardinality::O2M => {
            is_subj = false;
            break;
          }
          _ => {}
        }
      }
      
      if is_subj {
        subjs.push(a);
      }
    }

    if subjs.len() == 0 {
      let attr_ids: HashSet<usize> = attrs.iter().map(|x| *x).collect::<HashSet<_>>();
      
      // we have to try the external attributes
      for attr in &self.desc.attributes {
        if attr_ids.contains(&attr.id) {
          continue;
        }
        
        let mut is_candidate_subj = true;
        let mut covered_dims = HashSet::<usize>::new();
        
        for &ai in attrs {
          // we can only infer if there are any duplications if the alignments are dimensional
          // if they are dimension alignment, then the optimization engine must compress them into
          // just one alignment
          if self.aligns[attr.id][ai].len() != 1 || !self.aligns[attr.id][ai][0].is_range_align() {
            is_candidate_subj = false;
            break;
          }
          
          let align = &self.aligns[attr.id][ai][0].as_range_align();
          match align.compute_cardinality(&self.desc) {
            Cardinality::M2M => {
              is_candidate_subj = false;
              break;
            }
            Cardinality::O2M => {
              is_candidate_subj = false;
              break;
            }
            _ => {
              for ad in &align.aligned_dims {
                covered_dims.insert(ad.source_dim);
              }
            }
          }
        }
        
        if is_candidate_subj && covered_dims == attr.path.get_nary_steps().into_iter().collect::<HashSet<_>>() {
          // the second condition detect if there is duplication
          subjs.push(attr.id);
        }
      }
    }

    subjs
  }
  
  /// Perform inference to find all possible alignment functions
  pub fn inference(&mut self) {
    let mut mg = GraphMap::<usize, (), Directed>::default();
    for vid in 0..self.desc.attributes.len() {
      mg.add_node(vid);
    }
    
    for a in &self.desc.alignments {
      match a {
        Alignment::RangeAlign(da) => {
          mg.add_edge(da.source, da.target, ());
          mg.add_edge(da.target, da.source, ());
        }
        Alignment::ValueAlign(va) => {
          mg.add_edge(va.source, va.target, ());
          mg.add_edge(va.target, va.source, ());
        }
        Alignment::IdenticalAlign => unreachable!(),
      }
    }
    
    let mut n_new_edges;
    
    loop {
      // loop until no new edge has been found
      n_new_edges = 0;
      // infer more alignment functions, i.e. edges between nodes, using DFS
      for u0 in 0..self.desc.attributes.len() {
        let mut new_outgoing_edges = vec![];
        let mut new_incoming_edges = vec![];
        
        let mut dfs = CustomedDfs::new(&mg, u0);
        let mut revisit = FnvHashSet::default();

        if dfs.next(&mg, &revisit).is_some() {
          // call next first to skip the u0
          loop {
            // recording the length of the current stack
            // so that we know if we need to stop from exploring further from the next node
            // we can pop all of its children
            let stack_len = dfs.stack.len();
            let (u1, u2) = match dfs.next(&mg, &revisit) {
              None => break,
              Some((u1, u2)) => (u1, u2)
            };

            if mg.contains_edge(u0, u1) && !mg.contains_edge(u0, u2) {
              // try to infer alignment function between u0 and u2
              match self.infer_func(u0, u1, u2) {
                None => {
                  // haven't found any, hence we have to stop from exploring u2
                  // plus 1 because we take into account the u2 node, which was popped
                  for _ in 0..(dfs.stack.len() + 1 - stack_len) {
                    // remove all children of u2
                    dfs.stack.pop();
                  }
                  // mark this u2 as re-visited because it may be discovered from other nodes
                  // we should not have infinite recursive loop here
                  revisit.insert(u2);
                  continue;
                }
                Some(afuncs) => {
                  new_outgoing_edges.push(u2);
                  self.aligns[u0][u2] = afuncs;
                }
              }
              
              if let Some(afuncs) = self.infer_func(u2, u1, u0) {
                new_incoming_edges.push(u2);
                self.aligns[u2][u0] = afuncs;
              }
            }
          }
        }

        n_new_edges += new_incoming_edges.len() + new_outgoing_edges.len();
        
        for ui in new_outgoing_edges {
          mg.add_edge(u0, ui, ());
        }
        for ui in new_incoming_edges {
          mg.add_edge(ui, u0, ());
        }
      }
      
      if n_new_edges == 0 {
        // no more incoming edges
        break;
      }
    }
  }
  
  /// Infer an alignment function of xid and zid given alignments between (xid, yid) and (yid, zid)
  ///
  /// If there is only one way to join values of xid and zid, then the chain join will be the correct one
  pub fn infer_func(&self, xid: usize, yid: usize, zid: usize) -> Option<Vec<Alignment>> {
    let f = &self.aligns[xid][yid];
    let g = &self.aligns[yid][zid];
    
    let f_cardin = self.estimate_cardinality(f);
    let g_cardin = self.estimate_cardinality(g);
    
    // filter the case where we cannot chain these alignments
    let mut can_chain_alignments = false;
    match f_cardin {
      Cardinality::O2O => {
        can_chain_alignments = true;
      }
      Cardinality::O2M => {
        can_chain_alignments = true;
      }
      _ => {}
    };
    
    match g_cardin {
      Cardinality::O2O => {
        can_chain_alignments = true;
      }
      Cardinality::M2O => {
        can_chain_alignments = true;
      }
      _ => {}
    }

    if !can_chain_alignments {
      return None;
    }
    
    // chain them together
    let chained_joins = f.iter().chain(g.iter()).collect::<Vec<_>>();
    Some(self.optimize_chained_alignments(chained_joins))
  }
  
  /// Estimate
  pub fn estimate_cardinality(&self, aligns: &[Alignment]) -> Cardinality {
    let mut cardin = aligns[0].compute_cardinality(self.desc);
    // must always be > 0
    if aligns.len() <= 1 || cardin == Cardinality::M2M {
      return cardin;
    }
    
    for i in 1..aligns.len() {
      let cardin_i = aligns[i].compute_cardinality(self.desc);
      match cardin_i {
        Cardinality::O2O => {
          // do nothing, as this does not change the cardin
        }
        Cardinality::O2M => {
          match cardin {
            Cardinality::O2O => {
              cardin = Cardinality::O2M;
            }
            Cardinality::O2M => {
              cardin = Cardinality::O2M;
            }
            Cardinality::M2O => {
              // we don't know whether it is going to be M2M, O2O, O2M, M2O so we do a conservative prediction
              return Cardinality::M2M;
            }
            Cardinality::M2M => {
              return cardin;
            }
          }
        }
        Cardinality::M2O => {
          match cardin {
            Cardinality::O2O => {
              cardin = Cardinality::M2O;
            }
            Cardinality::O2M => {
              return Cardinality::M2M;
            }
            Cardinality::M2O => {
              cardin = Cardinality::M2O;
            }
            Cardinality::M2M => {
              return Cardinality::M2M;
            }
          }
        }
        Cardinality::M2M => {
          return cardin_i;
        }
      }
    }
    
    return cardin;
  }
  
  /// Optimize the chained joins
  fn optimize_chained_alignments(&self, aligns: Vec<&Alignment>) -> Vec<Alignment> {
    if aligns.len() == 0 {
      return vec![];
    }
    
    let mut joins = Vec::with_capacity(aligns.len());
    
    // rules 1: consecutive dimension alignments are combined together
    let mut aligns_iter = aligns.into_iter();
    joins.push(aligns_iter.next().unwrap().clone());
    
    for align in aligns_iter {
      if joins.last().unwrap().is_range_align() && align.is_range_align() {
        // we merge them together
        let merged_align = {
          let a0 = joins.pop().unwrap().into_range_align();
          let a1 = align.as_range_align();
          
          let a1map = a1.aligned_dims.iter()
            .map(|ad| (ad.source_dim, ad.target_dim))
            .collect::<HashMap<_, _>>();
          
          Alignment::RangeAlign(RangeAlignment {
            source: a0.source,
            target: a1.target,
            aligned_dims: a0.aligned_dims.iter()
              .filter(|ad| a1map.contains_key(&ad.target_dim))
              .map(|ad| {
                AlignedDim { source_dim: ad.source_dim, target_dim: a1map[&ad.target_dim] }
              })
              .collect::<Vec<_>>(),
          })
        };
        
        joins.push(merged_align);
      } else {
        joins.push(align.clone());
      }
    }
    
    joins
  }
}