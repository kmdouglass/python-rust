use pyo3::prelude::*;

use crate::python::par_iter_obj_collection_by_ref::{par_run, StateMachine, Transition};

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn python_lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<StateMachine>()?;
    m.add_class::<Transition>()?;
    m.add_function(wrap_pyfunction!(par_run, m)?)?;
    Ok(())
}
