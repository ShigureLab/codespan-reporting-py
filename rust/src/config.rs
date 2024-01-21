use pyo3::prelude::*;

#[pyclass]
pub struct Config {
    pub inner: codespan_reporting::term::Config,
}

#[pymethods]
impl Config {
    #[new]
    fn new() -> Self {
        Self {
            inner: codespan_reporting::term::Config::default(),
        }
    }
}
