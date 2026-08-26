use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan, stmt_ir::StmtIR};

#[derive(Debug, Clone)]
pub struct AsyncForIR {
    pub target: ExprIR,
    pub iter: ExprIR,
    pub body: Vec<StmtIR>,
    pub orelse: Vec<StmtIR>,
    pub type_comment: Option<String>,
    pub span: Option<SourceSpan>,
}
