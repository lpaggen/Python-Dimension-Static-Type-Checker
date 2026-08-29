use crate::ir::expr::ExprIR;

pub struct Raise<'a> {
    pub exception: Option<&'a ExprIR>,
    pub cause: Option<&'a ExprIR>,
}
