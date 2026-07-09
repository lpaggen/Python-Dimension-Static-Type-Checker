use crate::ir::nodes::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolIR {
    pub value: bool,
    pub span: Option<SourceSpan>,
}
