use crate::models::Location;
use crate::readers::{RAReader, Value};

pub struct FlattenFunc {
  location: Location,
}

impl FlattenFunc {
  pub fn exec<R: RAReader>(&self, reader: &mut R) {
    let mut iter = unsafe { (*(reader as *const R)).iter_data(&self.location) };
    loop {
      let idx = iter.value();
      let node = reader.get_mut_value(idx, 0);

      let mut result = vec![];
      match node {
        Value::Array(children) => {
          for c in children {
            result.append(c.as_mut_array());
          }
        }
        _ => panic!("ValueError: cannot flatten non-array node"),
      }

      reader.set_value(idx, 0, Value::Array(result));
      if !iter.advance() {
        break;
      }
    }
  }
}
