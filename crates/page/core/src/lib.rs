//! Core types and compilation pipeline for PAGE documents.

pub(crate) mod document;
pub(crate) mod gantt;
pub mod generator;
pub mod markdown;

mod errors;

pub use errors::CompileError;

/// Compiles Markdown source into a standalone PAGE HTML document.
///
/// The implementation will be added after the PAGE block syntax and
/// intermediate model contracts are finalized.
pub fn compile(_markdown: &str) -> Result<String, CompileError> {
    Err(CompileError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compilation_reports_that_the_pipeline_is_not_implemented() {
        assert_eq!(compile("# PAGE"), Err(CompileError::NotImplemented));
    }
}
