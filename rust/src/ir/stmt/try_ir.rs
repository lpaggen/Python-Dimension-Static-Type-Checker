use crate::ir::{nodes::ExceptHandlerIR, span_ir::SourceSpan, stmt_ir::StmtIR};

#[derive(Debug, Clone)]
pub struct TryIR {
    pub body: Vec<StmtIR>,
    pub handlers: Vec<ExceptHandlerIR>,
    pub orelse: Vec<StmtIR>,
    pub finalbody: Vec<StmtIR>,
    pub span: Option<SourceSpan>,
}
