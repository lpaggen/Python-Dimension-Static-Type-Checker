use crate::ir::nodes::span_ir::SourceSpan;


pub struct ScopeIR {
    pub id: i64,
    pub name: String,
    pub kind: String, // todo change to custom enum
    pub parent_id: i64,
    pub span: SourceSpan
}