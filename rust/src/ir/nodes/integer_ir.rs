use crate::ir::nodes::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntegerIR {
    pub value: i64,
    pub span: Option<SourceSpan>,
}
