use crate::ir::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoneIR {
    pub span: Option<SourceSpan>,
}
