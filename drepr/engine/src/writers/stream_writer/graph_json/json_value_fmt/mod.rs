use readers::prelude::Value;
use std::io::{BufWriter, Write};

pub mod int_value_fmt;
pub mod float_value_fmt;
pub mod str_value_fmt;
pub mod unspecified_value_fmt;

pub use self::int_value_fmt::*;
pub use self::float_value_fmt::*;
pub use self::str_value_fmt::*;
pub use self::unspecified_value_fmt::*;

/// The value formatter assume that the data is already in the form that complies with
/// the data type specified in the semantic model, and it's totally up to the implementation
/// to do that if they want (you should assume that they don't).
///
/// If you need a value formatter that does the check, you should enable the strict mode of the engine
/// which will handle checking the data type
pub trait JSONValueFmt<W: Write> {
  fn get_value(&self, val: &Value) -> String;
  fn write_value(&self, writer: &mut BufWriter<W>, val: &Value);
}