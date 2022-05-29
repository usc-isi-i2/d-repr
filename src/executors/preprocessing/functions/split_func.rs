use readers::path_expr::StepExpr;
use readers::prelude::{Index, PathExpr, RAReader, Value};

/// A split function is applied on each element of an array, if the function
/// return true, then we split the array at that element. If the function returns true for n
/// elements, it will split the arrays to n+1 sub-contiguous arrays.
///
/// Prerequisite:
/// 1. The function cannot be applied on non-contiguous array
///
/// # Examples:
///
/// Split arrays: [1, 2, 3, 4, 5]
/// If the results of the split function are:
///   1.
///     a. [false, true, false, true, false]
///     b. [[1, 2], [3, 4], [5]]
///     c. [[1], [2, 3], [4, 5]]
///   2.
///     a. [true, false, false, false, false]
///     b. [[1], [2, 3, 4, 5]]
///     c. [[1, 2, 3, 4, 5]]
///   3.
///     a. [true, true, false, true, true]
///     b. [[1], [2], [3, 4], [5]]
///     c. [[1], [2, 3], [4], [5]]
///
/// b is when the split value will be in previous array, c is when the split value will be in next array and
/// we have to filter out first empty array. We choose c because in practice b makes it harder to write splitting
/// function
pub struct SplitFunc<'a, F>
where
  F: FnMut(&Value, &[Index]) -> bool,
{
  pub path: &'a PathExpr,
  pub func: F,
}

impl<'a, F> SplitFunc<'a, F>
where
  F: FnMut(&Value, &[Index]) -> bool,
{
  /// We loop through each parent node of splitting nodes. For each parent node, apply the function
  /// to split children nodes of the current parent to get the result (nested array of children nodes)
  /// Then assigning the nested array to the parent node.
  ///
  /// Note that this function need to handle the case when we only split a subset of children nodes
  pub fn exec(&mut self, reader: &mut dyn RAReader) {
    // idempotent
    let range = match self.path.steps.last().unwrap() {
      StepExpr::Range(range) => {
        if range.step != 1 {
          panic!("Cannot split on non-contiguous array")
        }
        range
      },
      _ => {
        panic!("Can only apply split function on an array")
      }
    };

    let n_steps2parent = self.path.steps.len() - 1;
    let mut iter = unsafe { (*(reader as *const dyn RAReader)).iter_index(&self.path) };

    if n_steps2parent == 0 {
      // when the split is apply in the root level, we know the root level is an array, however, they
      // may have other pieces of information inside resource that support indexing
      let range_end = range.get_end(reader.len());
      let mut results = vec![vec![range.start]];
      for i in (range.start+1)..range_end {
        let idx = [Index::Idx(i)];
        let c = reader.get_value(&idx, 0);

        if (self.func)(&c, &idx) {
          // should split, add value to previous array, and add new array starting from this node

          // uncomment if we switch to `b` approach
//          results.last_mut().unwrap().push(i);
//          results.push(vec![]);

          // uncomment if we switch to `c` approach
          results.push(vec![i]);
        } else {
          // no split, add to previous array
          results.last_mut().unwrap().push(i);
        }
      }

      // uncomment if we switch to `b` approach
      // now, if we split on the last item, it add a new empty array and we have to remove it
//      if results.last().unwrap().len() == 0 {
//        results.pop();
//      }

      let n_sub_arrays = results.len();
      // assign the splitted array to the parent node
      for (i, val) in results.into_iter().enumerate() {
        let item = val.into_iter()
          .map(|j| std::mem::replace(reader.get_mut_value(&[Index::Idx(j)], 0), Value::Null))
          .collect::<Vec<_>>();
        reader.set_value(&[Index::Idx(range.start + i)], 0, Value::Array(item));
      }

      // remove redundant element in the reverse order
      for i in ((range.start+n_sub_arrays)..range_end).rev() {
        reader.remove(&Index::Idx(i));
      }
      return;
    }

    // freeze the last step so that we can loop through parent nodes
    iter.freeze_last_step();

    loop {
      let idx = iter.mut_value();
      let parent_node = reader.get_mut_value(&idx[..n_steps2parent], 0);

      // for each parent node, split the children nodes according to the split function
      match parent_node {
        Value::Array(children) => {
          let range_end = range.get_end(children.len());

          // the result vector of an split operator
          // init it to be one single array of first child node because it will be in the result anyway
          // the children[range.start] is now be Value::Null, which we don't care because we are going to
          // replace it soon
          let mut results = vec![vec![range.start]];
          // iter and swap the rest of the child nodes out of the parent node
          for i in (range.start+1)..range_end {
            // manually update the index of the node
            idx[n_steps2parent].set_idx(i);
            let c = &children[i];

            if (self.func)(&c, idx) {
              // should split, add value to previous array, and add new array starting from this node

              // uncomment if we switch to `b` approach
//              results.last_mut().unwrap().push(i);
//              results.push(vec![]);

              // uncomment if we switch to `c` approach
              results.push(vec![i]);
            } else {
              // no split, add to previous array
              results.last_mut().unwrap().push(i);
            }
          }

          // uncomment if we switch to `b` approach
          // now, if we split on the last item, it add a new empty array and we have to remove it
//          if results.last().unwrap().len() == 0 {
//            results.pop();
//          }

          let n_sub_arrays = results.len();
          // assign the splitted array to the parent node
          for (i, val) in results.into_iter().enumerate() {
            let item = val.into_iter()
              .map(|j| std::mem::replace(&mut children[j], Value::Null))
              .collect::<Vec<_>>();
            children[range.start+i] = Value::Array(item);
          }

          // remove redundant element in the reverse order
          children.drain((range.start+n_sub_arrays)..range_end);
        },
        _ => {
          // child nodes must be an array, then we can split, otherwise we cannot!
          panic!("Cannot apply split function on non-array");
        }
      }

      if !iter.advance() {
        break;
      }
    }
  }

  pub fn exec_non_idempotent(&mut self, reader: &mut dyn RAReader) {
    let range = match self.path.steps.last().unwrap() {
      StepExpr::Range(range) => {
        if range.step != 1 {
          panic!("Cannot split on non-contiguous array")
        }
        range
      },
      _ => {
        panic!("Can only apply split function on an array")
      }
    };

    let n_steps2parent = self.path.steps.len() - 1;
    let mut iter = unsafe { (*(reader as *const dyn RAReader)).iter_index(&self.path) };

    if n_steps2parent == 0 {
      // when the split is apply in the root level, we know the root level is an array, however, they
      // may have other pieces of information inside resource that support indexing
      let range_end = range.get_end(reader.len());
      let mut results = vec![vec![std::mem::replace(
        reader.get_mut_value(&[Index::Idx(range.start)], 0),
        Value::Null,
      )]];
      for i in (range.start+1)..range_end {
        let idx = [Index::Idx(i)];
        let c = std::mem::replace(reader.get_mut_value(&idx, 0), Value::Null);

        if (self.func)(&c, &idx) {
          // should split, add value to previous array, and add new array starting from this node
          results.last_mut().unwrap().push(c);
          results.push(vec![]);
        } else {
          // no split, add to previous array
          results.last_mut().unwrap().push(c);
        }
      }

      // now, if we split on the last item, it add a new empty array and we have to remove it
      if results.last().unwrap().len() == 0 {
        results.pop();
      }

      let n_sub_arrays = results.len();
      // assign the splitted array to the parent node
      for (i, val) in results.into_iter().enumerate() {
        reader.set_value(&[Index::Idx(range.start + i)], 0, Value::Array(val));
      }

      // remove redundant element in the reverse order
      for i in ((range.start+n_sub_arrays)..range_end).rev() {
        reader.remove(&Index::Idx(i));
      }
      return;
    }

    // freeze the last step so that we can loop through parent nodes
    iter.freeze_last_step();

    loop {
      let idx = iter.mut_value();
      let parent_node = reader.get_mut_value(&idx[..n_steps2parent], 0);

      // for each parent node, split the children nodes according to the split function
      match parent_node {
        Value::Array(children) => {
          let range_end = range.get_end(children.len());

          // the result vector of an split operator
          // init it to be one single array of first child node because it will be in the result anyway
          // the children[range.start] is now be Value::Null, which we don't care because we are going to
          // replace it soon
          let mut results = vec![vec![std::mem::replace(
            &mut children[range.start],
            Value::Null,
          )]];

          // iter and swap the rest of the child nodes out of the parent node
          for i in (range.start+1)..range_end {
            // manually update the index of the node
            idx[n_steps2parent].set_idx(i);
            let c = std::mem::replace(&mut children[i], Value::Null);

            if (self.func)(&c, idx) {
              // should split, add value to previous array, and add new array starting from this node
              results.last_mut().unwrap().push(c);
              results.push(vec![]);
            } else {
              // no split, add to previous array
              results.last_mut().unwrap().push(c);
            }
          }

          // now, if we split on the last item, it add a new empty array and we have to remove it
          if results.last().unwrap().len() == 0 {
            results.pop();
          }

          let n_sub_arrays = results.len();
          // assign the splitted array to the parent node
          for (i, val) in results.into_iter().enumerate() {
            children[range.start+i] = Value::Array(val);
          }

          // remove redundant element in the reverse order
          children.drain((range.start+n_sub_arrays)..range_end);
        },
        _ => {
          // child nodes must be an array, then we can split, otherwise we cannot!
          panic!("Cannot apply split function on non-array");
        }
      }

      if !iter.advance() {
        break;
      }
    }
  }
}
