use crate::models::Location;
use crate::models::*;
use crate::readers::*;

pub struct FilterFunc<'a, F>
where
  F: FnMut(&Value, &[Index]) -> bool,
{
  pub location: &'a Location,
  pub func: F,
}

impl<'a, F> FilterFunc<'a, F>
where
  F: FnMut(&Value, &[Index]) -> bool,
{
  pub fn exec<R: RAReader>(&mut self, reader: &mut R) {
    // we get the top-level, if the top-level
    // is the sub-reader, then we do a filter, and replace it.
    // it this actually feasible even if it is multi readers
    if self.location.slices.len() == 1 {
      match &self.location.slices[0] {
        Slice::Index(i) => {
          let idx = [i.idx.clone()];
          let val = reader.get_value(&idx, 0);
          if !(self.func)(val, &idx) {
            reader.remove(&i.idx);
          }
        }
        Slice::Range(r) => {
          let mut end = r.get_end(reader.len());
          end = end - end % r.step + r.step;

          let mut idx = [Index::Idx(r.start)];
          for i in (r.start..end).rev().step_by(r.step) {
            let val = reader.get_value(&idx, 0);
            idx[0] = Index::Idx(i);
            if !(self.func)(val, &idx) {
              reader.remove(&idx[0]);
            }
          }
        }
      }
    } else {
      let mut iter = unsafe { (*(reader as *const R)).iter_data(&self.location) };
      iter.freeze_last_index();

      match self.location.slices.last().unwrap() {
        Slice::Index(i) => loop {
          let idx = iter.value();
          let node = reader.get_mut_value(&idx[..idx.len() - 1], 0);
          if !(self.func)(node.get_child_value(&i.idx), idx) {
            node.remove(&i.idx);
          }

          if !iter.advance() {
            break;
          }
        },
        Slice::Range(r) => {
          // TODO: improve the logic here!
          if r.end.unwrap_or(-1) > 0 {
            let end = r.end.unwrap() as usize;
            let r_end = if end % r.start == 0 {
              end
            } else {
              end - end % r.start + r.step
            };

            loop {
              let idx = iter.mut_value();
              let node = reader.get_mut_value(&idx[..idx.len() - 1], 0);

              match node {
                Value::Array(children) => {
                  for i in (r.start..r_end).rev().step_by(r.step) {
                    idx[idx.len() - 1] = Index::Idx(i);
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
          } else {
            if r.step == 1 {
              loop {
                let idx = iter.mut_value();
                let node = reader.get_mut_value(&idx[..idx.len() - 1], 0);
                let r_end = r.get_end(node.len());
                match node {
                  Value::Array(children) => {
                    for i in (r.start..r_end).rev() {
                      idx[idx.len() - 1] = Index::Idx(i);
                      if !(self.func)(&children[i], idx) {
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
            } else {
              loop {
                let idx = iter.mut_value();
                let node = reader.get_mut_value(&idx[..idx.len() - 1], 0);

                let mut r_end = r.get_end(node.len());
                r_end = r_end - r_end % r.step + r.step;
                match node {
                  Value::Array(children) => {
                    for i in (r.start..r_end).rev().step_by(r.step) {
                      idx[idx.len() - 1] = Index::Idx(i);
                      if !(self.func)(&children[i], idx) {
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
          }
        }
      }
    }
  }
}
