use crate::ir::nodes::SourceSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringIR {
    pub value: String,
    pub span: Option<SourceSpan>,
}
