use crate::ir::nodes::annotation_ir::AnnotationIR;
use crate::ir::nodes::TypeParamIR;
use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan, stmt_ir::StmtIR};

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
    pub symbol_id: i64,
    pub arg: String,
    pub kind: ArgKind,
    pub annotation: Option<AnnotationIR>,
    pub default: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct ReturnIR {
    pub value: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct FunctionDefIR {
    pub id: i64,
    pub symbol_id: i64,
    pub name: String,
    pub scope_id: i64,
    pub body_scope_id: i64,
    pub args: Vec<ArgIR>,
    pub body: Vec<StmtIR>,
    pub returns: Option<AnnotationIR>,
    pub decorator_list: Vec<ExprIR>,
    pub type_comment: Option<String>,
    pub type_params: Vec<TypeParamIR>,
    pub span: Option<SourceSpan>,
}
