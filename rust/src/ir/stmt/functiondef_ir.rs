use crate::ir::nodes::TypeParamIR;
use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan, stmt_ir::StmtIR};
use crate::linker::symbol_ref::SymbolRef;

#[derive(Debug, Clone)]
pub enum ArgKind {
    PositionalOnly = 1,
    PositionalOrKeyword = 2,
    VarPositional = 3,
    KeywordOnly = 4,
    VarKeyword = 5,
}

impl TryFrom<i32> for ArgKind {
    type Error = String;

    fn try_from(int: i32) -> Result<Self, Self::Error> {
        match int {
            1 => Ok(ArgKind::PositionalOnly),
            2 => Ok(ArgKind::PositionalOrKeyword),
            3 => Ok(ArgKind::VarPositional),
            4 => Ok(ArgKind::KeywordOnly),
            5 => Ok(ArgKind::VarKeyword),
            _ => Err(format!("invalid ArgKind value: {}", int)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArgIR {
    pub symbol_id: usize,
    pub arg: String,
    pub kind: ArgKind,
    pub annotation: Option<ExprIR>,
    pub default: Option<Box<ExprIR>>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone)]
pub struct ReturnIR {
    pub value: Option<Box<ExprIR>>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone)]
pub struct FunctionDefIR {
    pub id: usize,
    pub symbol_id: usize,
    pub name: String,
    pub scope_id: usize,
    pub body_scope_id: usize,
    pub args: Vec<ArgIR>,
    pub body: Vec<StmtIR>,
    pub returns: Option<ExprIR>,
    pub decorator_list: Vec<ExprIR>,
    pub type_comment: Option<String>,
    pub type_params: Vec<TypeParamIR>,
    pub symbol_ref: SymbolRef,
    pub span: SourceSpan,
}
