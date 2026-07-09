use crate::ir::nodes::*;


#[derive(Debug, Clone)]
pub enum ExprIR {
    Identifier(IdentifierIR),
    Int(IntegerIR),
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
    // BoolOp(BoolOpIR),
    UnaryOp(UnaryOpIR),
    Compare(CompareIR),
    Call(CallExprIR),
}

impl ExprIR {
    pub fn span(&self) -> Option<SourceSpan> {
        match self {
            ExprIR::Identifier(node) => node.span,
            ExprIR::Int(node) => node.span,
            ExprIR::Float(node) => node.span,
            ExprIR::Bool(node) => node.span,
            ExprIR::String(node) => node.span,
            ExprIR::None(node) => node.span,

            ExprIR::List(node) => node.span,
            ExprIR::Tuple(node) => node.span,
            ExprIR::Slice(node) => node.span,
            ExprIR::Subscript(node) => node.span,
            ExprIR::AttributeExpr(node) => node.span,

            ExprIR::BinOp(node) => node.span,
            ExprIR::UnaryOp(node) => node.span,
            ExprIR::Compare(node) => node.span,
            ExprIR::Call(node) => node.span,
        }
    }
}
