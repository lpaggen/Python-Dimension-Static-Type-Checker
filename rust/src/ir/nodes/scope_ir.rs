use crate::ir::nodes::SourceSpan;

#[derive(Debug, Clone)]
pub struct ScopeIR {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub parent_id: Option<i64>,
    pub span: Option<SourceSpan>,
}
