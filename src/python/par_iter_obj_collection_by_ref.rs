//! Iterate over a list of Python objects in parallel without copying their data.
//!
use std::ops::DerefMut;

use pyo3::prelude::*;
use rand::prelude::*;
use rayon::prelude::*;

#[pyclass]
pub struct StateMachine {
    state: i32,
}

#[pymethods]
impl StateMachine {
    #[new]
    pub fn new() -> Self {
        StateMachine { state: 0 }
    }
}

#[pyclass(frozen)]
pub struct Transition {
    data: f64,
}

#[pymethods]
impl Transition {
    fn data(&self) -> f64 {
        self.data
    }
}

impl StateMachine {
    fn run(&mut self) -> Transition {
        let mut rng = rand::thread_rng();
        let time: f64 = rng.gen();

        self.state += 1;
        Transition { data: time }
    }
}

#[pyfunction]
pub fn par_run(machines: Vec<&PyCell<StateMachine>>) -> PyResult<Vec<Transition>> {
    let mut machines = machines
        .into_iter()
        .map(|cell| cell.try_borrow_mut())
        .collect::<Result<Vec<PyRefMut<StateMachine>>, _>>()?;

    let mut machines = machines
        .iter_mut()
        .map(|refr| refr.deref_mut())
        .collect::<Vec<&mut StateMachine>>();

    Ok(machines
        .par_iter_mut()
        .map(|machine| machine.run())
        .collect())
}
