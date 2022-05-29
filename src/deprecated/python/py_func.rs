use itertools::Itertools;

#[derive(Debug)]
pub struct PyFunc {
  pub resource_id: usize,
  pub code: String,
  pub call: String,
}

impl PyFunc {
  pub fn from(resource_id: usize, name: String, code: &str) -> PyFunc {
    let code = code.trim();
    let indent = PyFunc::detect_indent(code);

    let code = format!(
      "exec('''def {}(value, index, context):\n{}''')",
      name,
      code.split("\n")
        .map(|l| format!("{}{}", indent, l))
        .join("\n")
    );

    PyFunc {
      resource_id,
      call: format!("{}(value, index, context)", name),
      code,
    }
  }

  fn detect_indent(code: &str) -> String {
    let mut indent: String = "\t".to_string();

    for line in code.split("\n") {
      if line.starts_with("\t") {
        break;
      }

      if line.starts_with(" ") {
        let mut n = 0;
        for c in line.chars() {
          if c != ' ' {
            indent = " ".repeat(n);
            break;
          }
          n += 1;
        }
        break;
      }
    }

    indent
  }
}