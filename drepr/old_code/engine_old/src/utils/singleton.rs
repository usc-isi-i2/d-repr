use hashbrown::HashMap;

/// Hold a dict of objects identified by their names. The purpose of this struct is for
/// singleton pattern
pub struct Objects<T> {
  objects: HashMap<String, T>
}

impl<T> Objects<T> {
  pub fn new(inputs: Vec<(&str, T)>) -> Objects<T> {
    Objects {
      objects: inputs.into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
    }
  }

  pub fn get(&self, name: &str) -> &T {
    &self.objects[name]
  }
}