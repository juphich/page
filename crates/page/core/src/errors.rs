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
