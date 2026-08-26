use crate::ir::{
    expr_ir::ExprIR,
    nodes::{ArgIR, TypeParamIR},
    span_ir::SourceSpan,
    stmt_ir::StmtIR,
};

#[derive(Debug, Clone)]
pub struct AsyncFunctionDefIR {
    pub name: String,
    pub args: Vec<ArgIR>,
    pub body: Vec<StmtIR>,
    pub decorator_list: Vec<ExprIR>,
    pub returns: Option<ExprIR>,
    pub type_comment: Option<String>,
    pub scope_id: u64,
    pub type_params: Vec<TypeParamIR>,
    pub span: Option<SourceSpan>,
}
