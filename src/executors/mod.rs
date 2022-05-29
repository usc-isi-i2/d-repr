use serde::{Deserialize, Serialize};

use crate::execution_plans::{ClassesMapExecutionPlan, ExecutionPlan};
use crate::lang::Description;
use crate::writers::stream_writer::stream_writer::WriteResult;
use crate::writers::stream_writer::OutputFormat;

pub mod classes_map;
pub mod preprocessing;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PhysicalResource {
  #[serde(rename = "file")]
  File(String),
  #[serde(rename = "string")]
  String(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PhysicalOutput {
  #[serde(rename = "file")]
  File { fpath: String, format: OutputFormat },
  #[serde(rename = "memory")]
  Memory { format: OutputFormat },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Executor {
  pub resources: Vec<PhysicalResource>,
  pub output: PhysicalOutput,
  pub edges_optional: Vec<bool>,
  pub description: Description,
}

impl Executor {
  pub fn exec(&self) -> WriteResult {
    match self.get_exec_plan() {
      ExecutionPlan::ClassesMap(mut exec_plan) => classes_map::classes_map(
        &self.resources,
        &self.description,
        &mut exec_plan,
        &self.output,
      ),
    }
  }
  pub fn get_exec_plan(&self) -> ExecutionPlan {
    //    let edges_optional = vec![true; self.description.semantic_model.edges.len()];
    let output_format = match &self.output {
      PhysicalOutput::File { fpath: _, format } => format,
      PhysicalOutput::Memory { format } => format,
    };
    let exec_plan =
      ClassesMapExecutionPlan::new(&self.description, output_format, &self.edges_optional);
    ExecutionPlan::ClassesMap(exec_plan)
  }
}

impl PhysicalOutput {
  pub fn get_format(&self) -> &OutputFormat {
    match self {
      PhysicalOutput::File { fpath: _, format } => format,
      PhysicalOutput::Memory { format } => format,
    }
  }
}
