use pyo3::prelude::*;

use crate::meaning_of_life as mol;

#[pyfunction]
fn meaning_of_life() -> PyResult<i64> {
    Ok(mol())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn python_lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(meaning_of_life, m)?)?;
    Ok(())
}
