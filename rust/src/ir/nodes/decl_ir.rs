use crate::ir::nodes::{binding_ir::BindingIR, class_ir::ClassIR, function_ir::FunctionIR};
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub enum DeclIR {
    Binding(BindingIR),
    Function(FunctionIR),
    Class(ClassIR),
}

// impl DeclIR {
//     pub fn span(&self) -> Option<SourceSpan> {
//         match self {
//             Self::Binding(node) => node.span,
//             Self::Function(node) => node.span,
//             Self::Class(node) => node.span,
//         }
//     }
// }
