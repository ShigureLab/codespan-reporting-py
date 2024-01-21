use pyo3::prelude::*;
mod config;
mod diagnostic;
mod emit;
mod file;

/// A python binding for codespan-reporting.
#[pymodule]
#[pyo3(name = "_core")]
fn codespan_reporting_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(emit::emit, m)?)?;
    m.add_class::<file::SimpleFiles>()?;
    m.add_class::<diagnostic::Diagnostic>()?;
    m.add_class::<diagnostic::Severity>()?;
    m.add_class::<diagnostic::LabelStyle>()?;
    m.add_class::<diagnostic::Label>()?;
    m.add_class::<emit::StandardStream>()?;
    m.add_class::<config::Config>()?;
    Ok(())
}
