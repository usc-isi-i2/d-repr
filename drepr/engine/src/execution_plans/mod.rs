use serde::{Serialize};

pub use self::classes_map_plan::*;

pub mod classes_map_plan;
pub mod pseudo_id;

pub mod topological_sorting;

#[derive(Serialize)]
pub enum ExecutionPlan<'a> {
  ClassesMap(ClassesMapExecutionPlan<'a>)
}