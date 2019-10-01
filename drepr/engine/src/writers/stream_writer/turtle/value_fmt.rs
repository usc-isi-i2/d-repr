use std::io::{Write, BufWriter};

/// Write values in their corrected format to a channel
#[derive(Debug)]
pub struct ValueFmt {
  left: String,
  right: String,
  // when the type of values are string (they may need a double quote)
  left_string_type: String,
  right_string_type: String,
}

impl ValueFmt {
  /// Create a formatter for values with known type (so that `left` and `right` are
  /// constructed correctly, e.g., adding quote for string)
  pub fn specified_type(left: String, right: String) -> ValueFmt {
    ValueFmt {
      left_string_type: left.clone(),
      right_string_type: right.clone(),
      left,
      right,
    }
  }

  /// Create a formatter for values with unspecified type. The `left` and `right` are used for
  /// both string and non-string, hence the `left_string_type` and `right_string_type` need quoted
  pub fn unspecified_type(left: String, right: String) -> ValueFmt {
    ValueFmt {
      left_string_type: format!("{}\"", left),
      right_string_type: format!("\"{}", right),
      left,
      right,
    }
  }

  /// Write a value as it is string value
  #[inline]
  pub fn write_string_value<W: Write>(&self, channel: &mut BufWriter<W>, val: &str) {
    channel.write(self.left_string_type.as_bytes()).unwrap();
    channel.write(val.as_bytes()).unwrap();
    channel.write(self.right_string_type.as_bytes()).unwrap();
  }

  /// Write a value in its original format
  #[inline]
  pub fn write_value<W: Write>(&self, channel: &mut BufWriter<W>, val: &str) {
    channel.write(self.left.as_bytes()).unwrap();
    channel.write(val.as_bytes()).unwrap();
    channel.write(self.right.as_bytes()).unwrap();
  }

  /// Get a formatted value in a string format
  #[inline]
  #[allow(dead_code)]
  pub fn get_string_value(&self, val: &str) -> String {
    self.left_string_type.clone() + val + &self.right_string_type
  }

  /// Get a formatted value in its original format
  #[allow(dead_code)]
  #[inline]
  pub fn get_value(&self, val: &str) -> String {
    self.left.clone() + val + &self.right
  }
}