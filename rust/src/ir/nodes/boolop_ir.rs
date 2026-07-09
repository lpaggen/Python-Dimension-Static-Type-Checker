// use crate::ir::{ExprIR, Operator, SourceSpan};

// #[derive(Debug, Clone)]
// pub struct BoolOpIR {
//     pub values: Vec<ExprIR>,
//     pub op: Operator,
//     pub span: Option<SourceSpan>,
// }

// impl BoolOpIR {
//     pub fn binary(left: ExprIR, right: ExprIR, op: Operator, span: Option<SourceSpan>) -> Self {
//         Self {
//             values: vec![left, right],
//             op,
//             span,
//         }
//     }
// }
