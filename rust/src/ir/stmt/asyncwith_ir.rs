use crate::ir::{nodes::WithItemIR, span_ir::SourceSpan, stmt_ir::StmtIR};

#[derive(Debug, Clone)]
pub struct AsyncWithIR {
    pub items: Vec<WithItemIR>,
    pub body: Vec<StmtIR>,
    pub type_comment: Option<String>,
    pub span: Option<SourceSpan>,
}
