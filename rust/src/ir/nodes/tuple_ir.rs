use crate::ir::nodes::{ExprIR, SourceSpan};

#[derive(Debug, Clone)]
pub struct TupleIR {
    pub elements: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
