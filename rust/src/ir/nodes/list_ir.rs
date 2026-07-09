use crate::ir::nodes::{ExprIR, SourceSpan};

#[derive(Debug, Clone)]
pub struct ListIR {
    pub elements: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
