use crate::ir::{
    expr_ir::ExprIR,
    span_ir::SourceSpan,
    stmt::functiondef_ir::ArgIR,
};

#[derive(Debug, Clone)]
pub struct LambdaIR {
    pub args: Vec<ArgIR>,
    pub body: Box<ExprIR>,
    pub scope_id: i64,
    pub span: Option<SourceSpan>,
}
