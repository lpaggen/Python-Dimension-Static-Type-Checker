use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct KeywordIR {
    pub arg: Option<String>,
    pub value: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct CallIR {
    pub func: Box<ExprIR>,
    pub args: Vec<ExprIR>,
    pub keywords: Vec<KeywordIR>,
    pub span: Option<SourceSpan>,
}
