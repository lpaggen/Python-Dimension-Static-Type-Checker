use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan, stmt_ir::StmtIR};

#[derive(Debug, Clone)]
pub struct WhileIR {
    pub test: Box<ExprIR>,
    pub scope_id: i64,
    pub body_scope_id: i64,
    pub body: Vec<StmtIR>,
    pub orelse: Vec<StmtIR>,
    pub span: Option<SourceSpan>,
}
