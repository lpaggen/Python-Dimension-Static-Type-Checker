use crate::ir::{
    expr_ir::ExprIR,
    nodes::{KeywordIR, TypeParamIR},
    span_ir::SourceSpan,
    stmt_ir::StmtIR,
};

#[derive(Debug, Clone)]
pub struct ClassDefIR {
    pub id: i64,
    pub symbol_id: i64,
    pub name: String,
    pub scope_id: i64,
    pub body_scope_id: i64,
    pub body: Vec<StmtIR>,
    pub bases: Vec<ExprIR>,
    pub keywords: Vec<KeywordIR>,
    pub decorator_list: Vec<ExprIR>,
    pub type_params: Vec<TypeParamIR>,
    pub span: Option<SourceSpan>,
}
