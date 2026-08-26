use crate::ir::{expr_ir::ExprIR, nodes::ParamIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct TypeAliasIR {
    pub name: ExprIR,
    pub type_params: Vec<ParamIR>,
    pub value: ExprIR,
    pub span: Option<SourceSpan>,
}
