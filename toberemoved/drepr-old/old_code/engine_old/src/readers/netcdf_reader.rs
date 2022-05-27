use hashbrown::HashMap;
use netcdf;

use crate::iterators::StreamingIndexIterator;
use crate::models::{Location, Slice};
use crate::readers::ra_reader::{reader_iter_data, RAReader};

use super::{Index, Value};

#[derive(Debug)]
pub struct NetCDFRAReader {
    pub dataset: HashMap<String, Value>,
}

impl NetCDFRAReader {
    pub fn from_file(fpath: &str) -> NetCDFRAReader {
        let file = netcdf::open(fpath).unwrap();
        let mut dataset: HashMap<String, Value> = HashMap::with_capacity(file.root.variables.len());

        for (var_id, var) in file.root.variables.iter() {
            let value = if var.dimensions.len() == 0 {
                // primitive value
                if NetCDFRAReader::is_char(var.vartype) {
                    // this is known error: NetCDF: Attempt to convert between text & numbers
                    Value::Str("[ERROR] NetCDF: Attempt to convert between text & numbers (this is a known error)".to_string())
                } else if NetCDFRAReader::is_i64(var.vartype) {
                    Value::I64(
                        var.get_int64(true).expect(
                            "Should be able to read an integer from a primitive variable in NetCDF",
                        )[0],
                    )
                } else if NetCDFRAReader::is_f64(var.vartype) {
                    Value::F64(
                        var.get_double(true).expect(
                            "Should be able to read a double from a primitive variable in NetCDF",
                        )[0],
                    )
                } else {
                    panic!("Doesn't know how to handle data type: {}", var.vartype);
                }
            } else if var.dimensions.len() == 1 {
                if NetCDFRAReader::is_i64(var.vartype) {
                    Value::Array(var.get_int64(true).expect("Should be able to read an integer from a primitive variable in NetCDF")
                        .into_iter()
                        .map(|v| Value::I64(v))
                        .collect::<Vec<_>>())
                } else if NetCDFRAReader::is_f64(var.vartype) {
                    Value::Array(var.get_double(true).expect("Should be able to read a double from a primitive variable in NetCDF")
                        .into_iter()
                        .map(|v| Value::F64(v))
                        .collect::<Vec<_>>())
                } else {
                    panic!("Doesn't know how to handle data type: {}", var.vartype);
                }
            } else if var.dimensions.len() == 2 {
                if NetCDFRAReader::is_i64(var.vartype) {
                    let mut iter = var
                        .get_int64(true)
                        .expect(
                            "Should be able to read an integer from a primitive variable in NetCDF",
                        )
                        .into_iter();

                    let mut values = Vec::with_capacity(var.dimensions[0].len as usize);
                    for _ in 0..var.dimensions[0].len {
                        let mut row = Vec::with_capacity(var.dimensions[1].len as usize);
                        for _ in 0..var.dimensions[1].len {
                            row.push(Value::I64(iter.next().unwrap()));
                        }
                        values.push(Value::Array(row));
                    }

                    Value::Array(values)
                } else if NetCDFRAReader::is_f64(var.vartype) {
                    let mut iter = var
                        .get_double(true)
                        .expect(
                            "Should be able to read a double from a primitive variable in NetCDF",
                        )
                        .into_iter();

                    let mut values = Vec::with_capacity(var.dimensions[0].len as usize);
                    for _ in 0..var.dimensions[0].len {
                        let mut row = Vec::with_capacity(var.dimensions[1].len as usize);
                        for _ in 0..var.dimensions[1].len {
                            row.push(Value::F64(iter.next().unwrap()));
                        }
                        values.push(Value::Array(row));
                    }

                    Value::Array(values)
                } else {
                    panic!("Doesn't know how to handle data type: {}", var.vartype);
                }
            } else if var.dimensions.len() == 3 {
                if NetCDFRAReader::is_i64(var.vartype) {
                    let mut iter = var
                        .get_int64(true)
                        .expect(
                            "Should be able to read an integer from a primitive variable in NetCDF",
                        )
                        .into_iter();

                    let mut d0 = Vec::with_capacity(var.dimensions[0].len as usize);
                    for _ in 0..var.dimensions[0].len {
                        let mut d1 = Vec::with_capacity(var.dimensions[1].len as usize);
                        for _ in 0..var.dimensions[1].len {
                            let mut d2 = Vec::with_capacity(var.dimensions[2].len as usize);
                            for _ in 0..var.dimensions[2].len {
                                d2.push(Value::I64(iter.next().unwrap()));
                            }
                            d1.push(Value::Array(d2));
                        }
                        d0.push(Value::Array(d1));
                    }

                    Value::Array(d0)
                } else if NetCDFRAReader::is_f64(var.vartype) {
                    let mut iter = var
                        .get_double(true)
                        .expect(
                            "Should be able to read a double from a primitive variable in NetCDF",
                        )
                        .into_iter();

                    let mut d0 = Vec::with_capacity(var.dimensions[0].len as usize);
                    for _ in 0..var.dimensions[0].len {
                        let mut d1 = Vec::with_capacity(var.dimensions[1].len as usize);
                        for _ in 0..var.dimensions[1].len {
                            let mut d2 = Vec::with_capacity(var.dimensions[2].len as usize);
                            for _ in 0..var.dimensions[2].len {
                                d2.push(Value::F64(iter.next().unwrap()));
                            }
                            d1.push(Value::Array(d2));
                        }
                        d0.push(Value::Array(d1));
                    }

                    Value::Array(d0)
                } else {
                    panic!("Doesn't know how to handle data type: {}", var.vartype);
                }
            } else {
                panic!(
                    "Not implemented for {} dimension variable",
                    var.dimensions.len()
                );
            };

            dataset.insert(var_id.clone(), value);
        }

        NetCDFRAReader { dataset }
    }

    #[inline]
    pub fn is_i64(var_type: i32) -> bool {
        // https://github.com/mhiley/rust-netcdf/blob/master/netcdf-sys/src/netcdf_const.rs
        return (var_type != 2 && 4 >= var_type && var_type >= 1)
            || (11 >= var_type && var_type >= 7);
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
  fn into_value(self) -> Value {
    Value::Object(self.dataset)
  }

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
      self.dataset.get_mut(index[start_idx].as_str()).unwrap().get_mut_value(index, start_idx + 1)
    }
  }

  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value) {
    if index.len() - 1 == start_idx {
      self.dataset.insert(index[start_idx].as_str().to_string(), val);
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

  fn ground_location(&self, loc: &mut Location, start_idx: usize) {
    // we can only ground the first range slice
    let mut ptr = &self.dataset[loc.slices[start_idx].as_index().idx.as_str()];
    for s in &mut loc.slices[start_idx + 1..] {
      match s {
        Slice::Range(r) => {
          match r.end {
            None => {
              r.end = Some(ptr.len() as i64);
            },
            Some(e) => {
              if e < 0 {
                r.end = Some(ptr.len() as i64 + e);
              }
            }
          }
          break;
        }
        Slice::Index(i) => {
          ptr = ptr.get_child_value(&i.idx);
        }
      }
    }
  }

  fn can_change_value_type(&mut self) {}

  fn iter_data<'a>(&'a self, loc: &Location) -> Box<dyn StreamingIndexIterator + 'a> {
    reader_iter_data(self, loc)
  }
}