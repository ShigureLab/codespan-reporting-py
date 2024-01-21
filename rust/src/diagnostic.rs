use pyo3::prelude::*;

type FileId = usize;

#[pyclass]
#[derive(Clone)]
pub enum Severity {
    Bug,
    Error,
    Warning,
    Note,
    Help,
}

impl From<Severity> for codespan_reporting::diagnostic::Severity {
    fn from(severity: Severity) -> Self {
        match severity {
            Severity::Bug => codespan_reporting::diagnostic::Severity::Bug,
            Severity::Error => codespan_reporting::diagnostic::Severity::Error,
            Severity::Warning => codespan_reporting::diagnostic::Severity::Warning,
            Severity::Note => codespan_reporting::diagnostic::Severity::Note,
            Severity::Help => codespan_reporting::diagnostic::Severity::Help,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum LabelStyle {
    Primary,
    Secondary,
}

impl From<LabelStyle> for codespan_reporting::diagnostic::LabelStyle {
    fn from(label_style: LabelStyle) -> Self {
        match label_style {
            LabelStyle::Primary => codespan_reporting::diagnostic::LabelStyle::Primary,
            LabelStyle::Secondary => codespan_reporting::diagnostic::LabelStyle::Secondary,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Label {
    pub inner: codespan_reporting::diagnostic::Label<FileId>,
}

#[pymethods]
impl Label {
    #[new]
    fn new(
        label_style: LabelStyle,
        file_id: FileId,
        start: usize,
        end: usize,
        message: &str,
    ) -> Self {
        Self {
            inner: codespan_reporting::diagnostic::Label::new(
                label_style.into(),
                file_id,
                start..end,
            )
            .with_message(message),
        }
    }

    #[staticmethod]
    fn primary(file_id: FileId, start: usize, end: usize, message: &str) -> Self {
        Self::new(LabelStyle::Primary, file_id, start, end, message)
    }

    #[staticmethod]
    fn secondary(file_id: FileId, start: usize, end: usize, message: &str) -> Self {
        Self::new(LabelStyle::Secondary, file_id, start, end, message)
    }
}

#[pyclass]
pub struct Diagnostic {
    pub inner: codespan_reporting::diagnostic::Diagnostic<FileId>,
}

#[pymethods]
impl Diagnostic {
    #[new]
    #[pyo3(signature = (
        severity,
        code = None,
        message = "",
        labels = vec![],
        notes = vec![])
    )]
    fn new(
        severity: Severity,
        code: Option<&str>,
        message: &str,
        labels: Vec<Label>,
        notes: Vec<&str>,
    ) -> Self {
        Self {
            inner: codespan_reporting::diagnostic::Diagnostic::new(severity.into())
                .with_code(code.unwrap_or("").to_string())
                .with_message(message.to_string())
                .with_labels(labels.into_iter().map(|l| l.inner).collect())
                .with_notes(notes.into_iter().map(|n| n.to_string()).collect()),
        }
    }

    #[staticmethod]
    #[pyo3(signature = (code = None, message = "", labels = vec![], notes = vec![]))]
    fn bug(code: Option<&str>, message: &str, labels: Vec<Label>, notes: Vec<&str>) -> Self {
        Self::new(Severity::Bug, code, message, labels, notes)
    }

    #[staticmethod]
    #[pyo3(signature = (code = None, message = "", labels = vec![], notes = vec![]))]
    fn error(code: Option<&str>, message: &str, labels: Vec<Label>, notes: Vec<&str>) -> Self {
        Self::new(Severity::Error, code, message, labels, notes)
    }

    #[staticmethod]
    #[pyo3(signature = (code = None, message = "", labels = vec![], notes = vec![]))]
    fn warning(code: Option<&str>, message: &str, labels: Vec<Label>, notes: Vec<&str>) -> Self {
        Self::new(Severity::Warning, code, message, labels, notes)
    }

    #[staticmethod]
    #[pyo3(signature = (code = None, message = "", labels = vec![], notes = vec![]))]
    fn note(code: Option<&str>, message: &str, labels: Vec<Label>, notes: Vec<&str>) -> Self {
        Self::new(Severity::Note, code, message, labels, notes)
    }

    #[staticmethod]
    #[pyo3(signature = (code = None, message = "", labels = vec![], notes = vec![]))]
    fn help(code: Option<&str>, message: &str, labels: Vec<Label>, notes: Vec<&str>) -> Self {
        Self::new(Severity::Help, code, message, labels, notes)
    }
}
