use crate::ir::expr_ir::ExprIR;

#[derive(Debug, Clone)]
pub struct WithItemIR {
    pub context_expr: ExprIR,
    pub optional_vars: Option<ExprIR>,
}
