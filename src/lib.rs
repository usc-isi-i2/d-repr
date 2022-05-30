#[macro_use]
pub mod ops_macro;

pub mod alignments;
pub mod execution_plans;
pub mod executors;
pub mod lang;
pub mod python;
pub mod writers;

use pyo3::prelude::*;

use crate::python::{complete_description, Engine};

#[pymodule]
fn drepr(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Engine>()?;
    m.add_function(wrap_pyfunction!(
        complete_description::complete_description,
        m
    )?)?;
    Ok(())
}
