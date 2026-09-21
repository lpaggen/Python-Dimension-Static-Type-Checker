use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan, stmt_ir::StmtIR};

#[derive(Debug, Clone)]
pub struct ForIR {
    pub target: Box<ExprIR>,
    pub iter: Box<ExprIR>,
    pub scope_id: usize,
    pub body_scope_id: usize,
    pub body: Vec<StmtIR>,
    pub orelse: Vec<StmtIR>,
    pub span: SourceSpan,
}
