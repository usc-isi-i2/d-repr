mod map_func;
mod filter_func;
mod fold_func;
mod split_func;
mod flatten_func;
mod group_by_func;
pub mod built_ins;

pub use self::map_func::{MapFunc, MapInsertFunc};
pub use self::filter_func::FilterFunc;
pub use self::fold_func::FoldFunc;
pub use self::split_func::SplitFunc;
pub use self::flatten_func::FlattenFunc;

use crate::readers::RAReader;

pub trait PreprocessingFunc {
  fn exec<R: RAReader>(&self, reader: &mut R);
}