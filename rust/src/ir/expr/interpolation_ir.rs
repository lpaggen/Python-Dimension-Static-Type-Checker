use crate::ir::expr_ir::ExprIR;
use crate::ir::nodes::{Conversion, JoinedStrIR};
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct InterpolationIR {
    pub value: Box<ExprIR>,
    pub str: Option<String>,
    pub conversion: Conversion,
    pub format_spec: Option<JoinedStrIR>,
    pub span: Option<SourceSpan>,
}
