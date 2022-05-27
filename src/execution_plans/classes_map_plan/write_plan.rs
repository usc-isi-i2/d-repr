use serde::{Deserialize, Serialize};

use crate::writers::stream_writer::{WriteMode};

#[derive(Serialize, Deserialize, Debug)]
pub enum WritePlan {
  SingleWriter2File { class_write_modes: Vec<WriteMode> },
}