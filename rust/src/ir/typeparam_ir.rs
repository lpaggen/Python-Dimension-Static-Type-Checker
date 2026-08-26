use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub enum TypeParamIR {
    TypeVar(TypeVarIR),
    ParamSpec(ParamSpecIR),
    TypeVarTuple(TypeVarTupleIR),
}

#[derive(Debug, Clone)]
pub struct TypeVarIR {
    pub name: String,
    pub bound: Option<ExprIR>,
    pub default_value: Option<ExprIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct ParamSpecIR {
    pub name: String,
    pub default_value: Option<ExprIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct TypeVarTupleIR {
    pub name: String,
    pub default_value: Option<ExprIR>,
    pub span: Option<SourceSpan>,
}
