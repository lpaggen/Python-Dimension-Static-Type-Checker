use crate::ir::nodes::*;


#[derive(Debug, Clone)]
pub enum ExprIR {
    Identifier(IdentifierIR),
    Integer(IntegerIR),
    Float(FloatIR),
    Bool(BooleanIR),
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

// impl ExprIR {
//     pub fn span(&self) -> SourceSpan {
//         match self {
//             ExprIR::Float(node) => node.span,
//             ExprIR::Integer(node) => node.span,
//             ExprIR::Identifier(node) => node.span,
//             ExprIR::CallExpr(node) => node.span,
//             ExprIR::BinOp(node) => node.span,
//         }
//     }
// }