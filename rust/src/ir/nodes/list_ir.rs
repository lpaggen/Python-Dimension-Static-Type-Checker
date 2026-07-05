use crate::ir::{ExprIR, SourceSpan};

#[derive(Debug, Clone)]
pub struct ListIR {
    pub elements: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
