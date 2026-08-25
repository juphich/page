//! Core types and compilation pipeline for PAGE documents.

pub mod document;
pub mod gantt;
pub mod generator;
pub mod markdown;

/// Compiles Markdown source into a standalone PAGE HTML document.
///
/// The implementation will be added after the PAGE block syntax and
/// intermediate model contracts are finalized.
pub fn compile(_markdown: &str) -> Result<String, CompileError> {
    Err(CompileError::NotImplemented)
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompileError {
    NotImplemented,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotImplemented => formatter.write_str("PAGE compilation is not implemented yet"),
        }
    }
}

impl std::error::Error for CompileError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compilation_reports_that_the_pipeline_is_not_implemented() {
        assert_eq!(compile("# PAGE"), Err(CompileError::NotImplemented));
    }
}
