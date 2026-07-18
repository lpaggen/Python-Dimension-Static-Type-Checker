use crate::ir::nodes::{BooleanIR, FloatIR, IdentifierIR, IntegerIR, ListIR, NoneIR, SliceIR, StringIR, TupleIR, SubscriptIR, AttributeExprIR, BinOpIR, BoolOpIR, UnaryOpIR, CompareIR, CallExprIR};

#[derive(Debug, Clone)]
pub enum ExprIR {
    IdentifierExpr(IdentifierIR),
    IntegerExpr(IntegerIR),
    FloatExpr(FloatIR),
    BoolExpr(BooleanIR),
    StringExpr(StringIR),
    NoneExpr(NoneIR),

    ListExpr(ListIR),
    TupleExpr(TupleIR),
    SliceExpr(SliceIR),
    SubscriptExpr(SubscriptIR),
    AttributeExpr(AttributeExprIR),

    BinOpExpr(BinOpIR),
    BoolOpExpr(BoolOpIR),
    UnaryOpExpr(UnaryOpIR),
    CompareExpr(CompareIR),
    CallExpr(CallExprIR),
}

// impl ExprIR {
//     pub fn span(&self) -> &SourceSpan {
//         match self {
//             ExprIR::Float(node) => node.get_span(),
//             ExprIR::Integer(node) => &node.span,
//             ExprIR::Identifier(node) => &node.span,
//             ExprIR::CallExpr(node) => &node.span,
//             ExprIR::BinOp(node) => &node.span,
//         }
//     }
// }