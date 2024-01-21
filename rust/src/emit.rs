use pyo3::prelude::*;

use crate::{config::Config, diagnostic::Diagnostic, file::SimpleFiles};

#[pyclass]
#[derive(Clone)]
pub enum StandardStream {
    Stdout,
    Stderr,
}

impl From<StandardStream> for codespan_reporting::term::termcolor::StandardStream {
    fn from(stream: StandardStream) -> Self {
        match stream {
            StandardStream::Stdout => {
                Self::stdout(codespan_reporting::term::termcolor::ColorChoice::Auto)
            }
            StandardStream::Stderr => {
                Self::stderr(codespan_reporting::term::termcolor::ColorChoice::Auto)
            }
        }
    }
}

#[pyfunction]
pub fn emit(
    writer: &StandardStream,
    config: &Config,
    files: &SimpleFiles,
    diagnostic: &Diagnostic,
) -> PyResult<()> {
    let writer: codespan_reporting::term::termcolor::StandardStream = writer.clone().into();
    codespan_reporting::term::emit(
        &mut writer.lock(),
        &config.inner,
        &files.inner,
        &diagnostic.inner,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    Ok(())
}
