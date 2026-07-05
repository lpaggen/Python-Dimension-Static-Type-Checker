use crate::ir::nodes::*;


#[derive(Debug, Clone)]
pub enum ExprIR {
    Identifier(IdentifierIR),
    Integer(IntegerIR),
    Float(FloatIR),
    Bool(BoolIR),
    String(StringIR),
    None(NoneIR),

    List(ListIR),
    Tuple(TupleIR),
    Slice(SliceIR),
    Subscript(SubscriptIR),
    AttributeExpr(AttributeExprIR),

    BinOp(BinOpIR),
    BoolOp(BoolOpIR),
    UnaryOp(UnaryOpIR),
    Compare(CompareIR),
    CallExpr(CallExprIR),
}

impl ExprIR {
    pub fn span(&self) -> SourceSpan {
        match self {
            ExprIR::Float(node) => node.span,
            ExprIR::Int(node) => node.span,
            ExprIR::Name(node) => node.span,
            ExprIR::Call(node) => node.span,
            ExprIR::Binary(node) => node.span,
        }
    }
}