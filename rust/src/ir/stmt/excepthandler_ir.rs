use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan, stmt_ir::StmtIR};

#[derive(Debug, Clone)]
pub struct ExceptHandlerIR {
    pub exception_type: Option<ExprIR>,
    pub name: Option<String>,
    pub body: Vec<StmtIR>,
    pub span: Option<SourceSpan>,
}
