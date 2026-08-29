use crate::ir::expr::ExprIR;

#[derive(Debug, Clone)]
pub struct Raise<'a> {
    pub exception: Option<&'a ExprIR>,
    pub cause: Option<&'a ExprIR>,
}
