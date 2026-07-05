use crate::ir::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BooleanIR {
    pub value: bool,
    pub span: Option<SourceSpan>,
}
