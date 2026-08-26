use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan, stmt_ir::StmtIR};

#[derive(Debug, Clone)]
pub struct IfIR {
    pub test: Box<ExprIR>,
    pub scope_id: i64,
    pub then_scope_id: i64,
    pub else_scope_id: i64,
    pub body: Vec<StmtIR>,
    pub orelse: Vec<StmtIR>,
    pub span: Option<SourceSpan>,
}
