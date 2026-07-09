use crate::ir::nodes::{ExprIR, SourceSpan, StmtIR};

#[derive(Debug, Clone)]
pub struct ClassIR {
    pub id: i64,
    pub symbol_id: i64,
    pub name: String,
    pub scope_id: i64,
    pub body_scope_id: i64,
    pub body: Vec<StmtIR>,
    pub bases: Vec<ExprIR>,
    pub decorators: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
