use crate::ir::expr_ir::ExprIR;
use crate::ir::nodes::Conversion;
use crate::ir::span_ir::SourceSpan;

// #[derive(Debug, Clone)]
// pub enum JoinedStrValueIR {
//     FormattedValue(FormattedValueIR),
//     Constant(ConstantIR),
// }

#[derive(Debug, Clone)]
pub struct JoinedStrIR {
    pub values: Vec<ExprIR>, // this can only be Constant or FormattedValue, both are ExprIR
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct FormattedValueIR {
    pub value: Box<ExprIR>,
    pub conversion: Conversion,
    pub format_spec: Option<JoinedStrIR>,
    pub span: Option<SourceSpan>,
}
