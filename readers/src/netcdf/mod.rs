use crate::index::Index;
use crate::iterators::*;
use crate::path_expr::{PathExpr, StepExpr};
use crate::prelude::RAReader;
use crate::ra_reader::default_iter_index;
use crate::value::Value;
use hashbrown::HashMap;
use netcdf;
use netcdf::types::BasicType;

#[derive(Debug)]
pub struct NetCDFRAReader {
  pub dataset: HashMap<String, Value>,
}

impl NetCDFRAReader {
  pub fn from_file(fpath: &str) -> NetCDFRAReader {
    let file = netcdf::open(fpath).unwrap();
    let mut dataset: HashMap<String, Value> = HashMap::with_capacity(file.variables().count());

    for var in file.variables() {
      let var_id = var.name();
      let vartype = var.vartype();
      let vardims = var.dimensions();
      let value = if vardims.len() == 0 {
        // primitive value
        if vartype.as_basic().map_or(false, BasicType::is_char) {
          // this is known error: NetCDF: Attempt to convert between text & numbers
          Value::Str(
            "[ERROR] NetCDF: Attempt to convert between text & numbers (this is a known error)"
              .to_string(),
          )
        } else if vartype.is_i64() {
          Value::I64(
            var
              // .get_int64(true)
              .values(Option::None, Option::None)
              .expect("Should be able to read an integer from a primitive variable in NetCDF")[0],
          )
        } else if vartype.is_f64() {
          Value::F64(
            var
              .values(Option::None, Option::None)
              .expect("Should be able to read a double from a primitive variable in NetCDF")[0],
          )
        } else {
          panic!("Doesn't know how to handle data type: {:?}", vartype);
        }
      } else if vardims.len() == 1 {
        if vartype.is_i64() {
          Value::Array(
            var
              .values(Option::None, Option::None)
              .expect("Should be able to read an integer from a primitive variable in NetCDF")
              .into_iter()
              .map(|v| Value::I64(v))
              .collect::<Vec<_>>(),
          )
        } else if vartype.is_f64() {
          Value::Array(
            var
              .values(Option::None, Option::None)
              .expect("Should be able to read a double from a primitive variable in NetCDF")
              .into_iter()
              .map(|v| Value::F64(v))
              .collect::<Vec<_>>(),
          )
        } else {
          panic!("Doesn't know how to handle data type: {:?}", vartype);
        }
      } else if vardims.len() == 2 {
        if vartype.is_i64() {
          let mut iter = var
            .values(Option::None, Option::None)
            .expect("Should be able to read an integer from a primitive variable in NetCDF")
            .into_iter();

          let mut values = Vec::with_capacity(vardims[0].len() as usize);
          for _ in 0..vardims[0].len() {
            let mut row = Vec::with_capacity(vardims[1].len() as usize);
            for _ in 0..vardims[1].len() {
              row.push(Value::I64(iter.next().unwrap()));
            }
            values.push(Value::Array(row));
          }

          Value::Array(values)
        } else if vartype.is_f64() {
          let mut iter = var
            .values(Option::None, Option::None)
            .expect("Should be able to read a double from a primitive variable in NetCDF")
            .into_iter();

          let mut values = Vec::with_capacity(vardims[0].len() as usize);
          for _ in 0..vardims[0].len() {
            let mut row = Vec::with_capacity(vardims[1].len() as usize);
            for _ in 0..vardims[1].len() {
              row.push(Value::F64(iter.next().unwrap()));
            }
            values.push(Value::Array(row));
          }

          Value::Array(values)
        } else {
          panic!("Doesn't know how to handle data type: {:?}", vartype);
        }
      } else if vardims.len() == 3 {
        if vartype.is_i64() {
          let mut iter = var
            .values(Option::None, Option::None)
            .expect("Should be able to read an integer from a primitive variable in NetCDF")
            .into_iter();

          let mut d0 = Vec::with_capacity(vardims[0].len() as usize);
          for _ in 0..vardims[0].len() {
            let mut d1 = Vec::with_capacity(vardims[1].len() as usize);
            for _ in 0..vardims[1].len() {
              let mut d2 = Vec::with_capacity(vardims[2].len() as usize);
              for _ in 0..vardims[2].len() {
                d2.push(Value::I64(iter.next().unwrap()));
              }
              d1.push(Value::Array(d2));
            }
            d0.push(Value::Array(d1));
          }

          Value::Array(d0)
        } else if vartype.is_f64() {
          let mut iter = var
            .values(Option::None, Option::None)
            .expect("Should be able to read a double from a primitive variable in NetCDF")
            .into_iter();

          let mut d0 = Vec::with_capacity(vardims[0].len() as usize);
          for _ in 0..vardims[0].len() {
            let mut d1 = Vec::with_capacity(vardims[1].len() as usize);
            for _ in 0..vardims[1].len() {
              let mut d2 = Vec::with_capacity(vardims[2].len() as usize);
              for _ in 0..vardims[2].len() {
                d2.push(Value::F64(iter.next().unwrap()));
              }
              d1.push(Value::Array(d2));
            }
            d0.push(Value::Array(d1));
          }

          Value::Array(d0)
        } else {
          panic!("Doesn't know how to handle data type: {:?}", vartype);
        }
      } else {
        panic!("Not implemented for {} dimension variable", vardims.len());
      };

      dataset.insert(var.name(), value);
    }

    NetCDFRAReader { dataset }
  }

  #[inline]
  pub fn is_i64(var_type: i32) -> bool {
    // https://github.com/mhiley/rust-netcdf/blob/master/netcdf-sys/src/netcdf_const.rs
    return (var_type != 2 && 4 >= var_type && var_type >= 1) || (11 >= var_type && var_type >= 7);
  }

  #[inline]
  pub fn is_f64(var_type: i32) -> bool {
    // https://github.com/mhiley/rust-netcdf/blob/master/netcdf-sys/src/netcdf_const.rs
    return var_type == 5 || var_type == 6; // float or double
  }

  #[inline]
  pub fn is_char(var_type: i32) -> bool {
    return var_type == 2;
  }
}

impl RAReader for NetCDFRAReader {
  fn get_value(&self, index: &[Index], start_idx: usize) -> &Value {
    if start_idx == index.len() - 1 {
      &self.dataset[index[start_idx].as_str()]
    } else {
      self.dataset[index[start_idx].as_str()].get_value(index, start_idx + 1)
    }
  }

  fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value {
    if start_idx == index.len() - 1 {
      self.dataset.get_mut(index[start_idx].as_str()).unwrap()
    } else {
      self
        .dataset
        .get_mut(index[start_idx].as_str())
        .unwrap()
        .get_mut_value(index, start_idx + 1)
    }
  }

  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value) {
    if index.len() - 1 == start_idx {
      self
        .dataset
        .insert(index[start_idx].as_str().to_string(), val);
      return;
    }

    self
      .dataset
      .get_mut(index[start_idx].as_str())
      .unwrap()
      .set_value(&index, start_idx + 1, val)
  }

  fn len(&self) -> usize {
    self.dataset.len()
  }

  fn remove(&mut self, index: &Index) {
    self.dataset.remove(index.as_str());
  }

  fn ground_path(&self, path: &mut PathExpr, start_idx: usize) {
    // we can only ground the first range slice
    let mut ptr = &self.dataset[path.steps[start_idx].as_index().val.as_str()];
    for s in &mut path.steps[start_idx + 1..] {
      match s {
        StepExpr::Range(r) => {
          match r.end {
            None => {
              r.end = Some(ptr.len() as i64);
            }
            Some(e) => {
              if e < 0 {
                r.end = Some(ptr.len() as i64 + e);
              }
            }
          }
          break;
        }
        StepExpr::Index(i) => {
          ptr = ptr.get_child_value(&i.val);
        }
        _ => unimplemented!(),
      }
    }
  }

  fn iter_index<'a>(&'a self, path: &PathExpr) -> Box<dyn IndexIterator + 'a> {
    default_iter_index(self, path)
  }
}
