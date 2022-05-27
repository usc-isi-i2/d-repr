use readers::path_expr::StepExpr;
use readers::prelude::{Index, PathExpr, RAReader, Value};

pub struct MapFunc<'a, F>
  where
    F: FnMut(&mut Value, &[Index]) -> Value,
{
  pub path: &'a PathExpr,
  pub func: F,
}

impl<'a, F> MapFunc<'a, F>
  where
    F: FnMut(&mut Value, &[Index]) -> Value,
{
  pub fn exec(&mut self, reader: &mut dyn RAReader) {
    let mut iter = unsafe { (*(reader as *const dyn RAReader)).iter_index(&self.path) };
    if self.path.steps.len() == 1 {
      loop {
        let idx = iter.value();
        let val = (self.func)(reader.get_mut_value(idx, 0), &idx);
        reader.set_value(idx, 0, val);
        if !iter.advance() {
          break;
        }
      }
    } else {
      match self.path.steps.last().unwrap() {
        StepExpr::Index(_) => loop {
          let idx = iter.value();
          let val = (self.func)(reader.get_mut_value(idx, 0), &idx);
          reader.set_value(idx, 0, val);
          if !iter.advance() {
            break;
          }
        },
        StepExpr::Range(r) => {
          iter.freeze_last_step();
          loop {
            let idx = iter.mut_value();
            let node = reader.get_mut_value(&idx[..idx.len() - 1], 0);
            
            match node {
              Value::Array(children) => {
                let end = r.get_end(children.len());
                for i in (r.start..end).step_by(r.step) {
                  idx[idx.len() - 1] = Index::Idx(i);
                  children[i] = (self.func)(&mut children[i], idx);
                }
              }
              _ => panic!("ValueError: TODO: make a meaningful error message"),
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
}

//pub struct MapInsertFunc<'a, F>
//where
//  F: FnMut(&Value, &[Index]) -> Value
//{
//  pub path: &'a PathExpr,
//  pub output: String,
//  pub func: F
//}
//
//impl<'a, F> MapInsertFunc<'a, F>
//where
//  F: FnMut(&Value, &[Index]) -> Value
//{
//  pub fn exec<R: RAReader>(&mut self, reader: &mut R) -> Value {
//    if self.path.get_unbounded_dims().len() == 0 {
//      let idx = self.path.get_first_index();
//      let val = reader.get_value(&idx, 0);
//
//      return (self.func)(val, &idx);
//    }
//
//    let mut iter = InsertIterator::new(reader, &self.path);
//    loop {
//      let val = (self.func)(iter.value(), iter.index());
//      iter.push(val);
//      if !iter.advance() {
//        break;
//      }
//    }
//    iter.get_data()
//  }
//}
