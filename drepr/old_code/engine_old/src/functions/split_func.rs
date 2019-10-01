use crate::models::Location;
use crate::readers::{Index, RAReader, Value};

pub struct SplitFunc<F>
where
  F: Fn(&Value, &[Index]) -> bool,
{
  location: Location,
  func: F,
}

impl<F> SplitFunc<F>
where
  F: Fn(&Value, &[Index]) -> bool,
{
  pub fn exec<R: RAReader>(&self, reader: &mut R) {
    let mut iter = unsafe { (*(reader as *const R)).iter_data(&self.location) };
    iter.freeze_last_index();

    let range = self.location.slices.last().unwrap().as_range();
    if range.step != 1 {
      panic!("Cannot split non-contiguous array");
    }

    let n_idx_sub_1 = self.location.slices.len() - 1;

    loop {
      let idx = iter.mut_value();
      let node = reader.get_mut_value(&idx[..n_idx_sub_1], 0);

      match node {
        Value::Array(children) => {
          let r_end = range.get_end(children.len());

          // the last index should be an non-empty array because we are splitting an array based on its item
          // the first piece of an array cannot be empty (even if the split function return true for that
          let mut results = vec![vec![std::mem::replace(
            &mut children[range.start],
            Value::Null,
          )]];

          let mut i = range.start + 1;
          for c in children.drain((range.start + 1)..r_end) {
            idx[n_idx_sub_1].set_idx(i);

            if (self.func)(&c, idx) {
              // we should split it
              results.push(vec![c]);
            } else {
              // we add to the last
              results.last_mut().unwrap().push(c);
            }

            i += 1;
          }

          // last index should not be empty
          if results.last().unwrap().len() == 0 {
            results.pop();
          }

          children[range.start] =
            Value::Array(results.into_iter().map(|x| Value::Array(x)).collect());
        }
        _ => panic!("ValueError: cannot flatten non-array node"),
      }
    }
  }
}
