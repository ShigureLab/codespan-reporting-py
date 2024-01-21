use pyo3::prelude::*;

#[pyclass]
pub struct SimpleFiles {
    pub inner: codespan_reporting::files::SimpleFiles<String, String>,
}

#[pymethods]
impl SimpleFiles {
    #[new]
    fn new() -> Self {
        Self {
            inner: codespan_reporting::files::SimpleFiles::new(),
        }
    }

    fn add(&mut self, name: &str, source: &str) -> usize {
        self.inner.add(name.to_string(), source.to_string())
    }
}
