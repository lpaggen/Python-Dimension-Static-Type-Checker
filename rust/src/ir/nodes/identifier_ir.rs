use crate::ir::nodes::SourceSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentifierIR {
    pub name: String,
    pub use_scope_id: i64,
    pub span: Option<SourceSpan>,
}
