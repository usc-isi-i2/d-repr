use crate::models::Location;
use crate::readers::{RAReader, Value, Index};

/// This function is applied on
///
pub struct FoldFunc<F>
where
  F: Fn(&Value, &Value, &[Index]) -> Value
{
  location: Location,
  initial: Value,
  func: F
}

impl<F> FoldFunc<F>
where
  F: Fn(&Value, &Value, &[Index]) -> Value
{
  pub fn exec<R: RAReader>(&self, reader: &mut R) {
    let mut iter = unsafe { (*(reader as *const R)).iter_data(&self.location) };
    let mut result = self.initial.clone();

    loop {
      let idx = iter.value();
      let val = reader.get_value(idx, 0);

      result = (self.func)(&result, val, idx);
      if !iter.advance() {
        break;
      }
    }
  }
}