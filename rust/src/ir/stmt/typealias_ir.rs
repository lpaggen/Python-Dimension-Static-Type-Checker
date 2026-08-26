use crate::ir::{expr_ir::ExprIR, nodes::TypeParamIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct TypeAliasIR {
    pub name: ExprIR,
    pub type_params: Vec<TypeParamIR>,
    pub value: ExprIR,
    pub span: Option<SourceSpan>,
}
