use readers::prelude::{Index, RAReader, Value, PathExpr};

use readers::path_expr::StepExpr;

pub struct FilterFunc<'a, F>
  where
    F: FnMut(&Value, &[Index]) -> bool,
{
  pub path: &'a PathExpr,
  pub func: F,
}

impl<'a, F> FilterFunc<'a, F>
  where
    F: FnMut(&Value, &[Index]) -> bool,
{
  pub fn exec(&mut self, reader: &mut dyn RAReader) {
    // we get the top-level, if the top-level
    // is the sub-reader, then we do a filter, and replace it.
    // it this actually feasible even if it is multi readers
    if self.path.steps.len() == 1 {
      match &self.path.steps[0] {
        StepExpr::Index(i) => {
          let idx = [i.val.clone()];
          let val = reader.get_value(&idx, 0);
          if !(self.func)(val, &idx) {
            reader.remove(&i.val);
          }
        }
        StepExpr::Range(r) => {
          let end = r.get_end(reader.len());
          let mut idx = [Index::Idx(r.start)];
          for i in (r.start..end).step_by(r.step).rev() {
            idx[0] = Index::Idx(i);
            let val = reader.get_value(&idx, 0);
            if !(self.func)(val, &idx) {
              reader.remove(&idx[0]);
            }
          }
        }
        StepExpr::SetIndex(_) => unimplemented!(),
        StepExpr::Wildcard => unimplemented!(),
      }
      return;
    }
    let mut iter = unsafe { (*(reader as *const dyn RAReader)).iter_index(&self.path) };
    iter.freeze_last_step();

    match self.path.steps.last().unwrap() {
      StepExpr::Index(i) => loop {
        let idx = iter.value();
        let node = reader.get_mut_value(&idx[..idx.len() - 1], 0);
        if !(self.func)(node.get_child_value(&i.val), idx) {
          node.remove(&i.val);
        }

        if !iter.advance() {
          break;
        }
      },
      StepExpr::Range(r) => {
        let n_steps2parent = self.path.steps.len() - 1;
        loop {
          let idx = iter.mut_value();
          let parent_node = reader.get_mut_value(&idx[..n_steps2parent], 0);

          match parent_node {
            Value::Array(children) => {
              let r_end = r.get_end(children.len());
              for i in (r.start..r_end).step_by(r.step).rev() {
                idx[n_steps2parent] = Index::Idx(i);
                if !(self.func)(&children[i], &idx) {
                  children.remove(i);
                }
              }
            }
            _ => panic!("ValueError: cannot filter array items of non-array value"),
          }

          if !iter.advance() {
            break;
          }
        }
      }
      StepExpr::SetIndex(_) => unimplemented!(),
      StepExpr::Wildcard => unimplemented!(),
    }
  }
}
