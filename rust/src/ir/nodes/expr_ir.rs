use crate::ir::{
    AttributeExprIR, BinOpIR, BoolOpIR, BooleanIR, CallExprIR, CompareIR, FloatIR, IdentifierIR,
    IntegerIR, ListIR, NoneIR, SliceIR, SourceSpan, StringIR, SubscriptIR, TupleIR, UnaryOpIR,
};

#[derive(Debug, Clone)]
pub enum ExprIR {
    Identifier(IdentifierIR),
    Integer(IntegerIR),
    Float(FloatIR),
    Boolean(BooleanIR),
    String(StringIR),
    NoneLit(NoneIR),

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
    pub fn span(&self) -> Option<SourceSpan> {
        match self {
            Self::Identifier(node) => node.span,
            Self::Integer(node) => node.span,
            Self::Float(node) => node.span,
            Self::Boolean(node) => node.span,
            Self::String(node) => node.span,
            Self::NoneLit(node) => node.span,

            Self::List(node) => node.span,
            Self::Tuple(node) => node.span,
            Self::Slice(node) => node.span,
            Self::Subscript(node) => node.span,
            Self::AttributeExpr(node) => node.span,

            Self::BinOp(node) => node.span,
            Self::BoolOp(node) => node.span,
            Self::UnaryOp(node) => node.span,
            Self::Compare(node) => node.span,
            Self::CallExpr(node) => node.span,
        }
    }
}
