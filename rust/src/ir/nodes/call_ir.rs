use crate::ir::nodes::span_ir::SourceSpan;
use crate::ir::nodes::expr_ir::ExprIR;


#[derive(Debug, Clone)]
pub struct CallExprIR {
    pub function: Box<ExprIR>,
    pub args: Vec<ExprIR>,
    pub keywords: Vec<KeywordArgIR>,
    pub span: SourceSpan,
}


#[derive(Debug, Clone)]
pub struct KeywordArgIR {
    pub name: String, 
    pub value: Box<ExprIR>,
    pub span: SourceSpan
}