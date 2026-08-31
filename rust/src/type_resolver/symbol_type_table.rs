use crate::diagnostic::diagnostic::Diagnostic;
use crate::ir::stmt::AnnAssignIR;
use crate::ir::stmt::StmtIR;
use std::collections::HashMap;

use crate::ir::expr_ir::ConstantIR;
use crate::ir::expr_ir::ExprIR;
use crate::linker::global_scope_table::GlobalSymbolTable;
use crate::linker::resolution_table::ResolutionTable;
use crate::linker::resolved_target::ResolvedTarget;
use crate::types::types::DimType;
use crate::types::types::TensorTypeState;
use crate::types::types::Type;
use crate::{
    linker::{symbol_ref::SymbolRef},
};

// probably won't need 'by_ref' since this struct might just be owned by the CFG pass in a later build
pub struct SymbolTypeTable {
    pub by_ref: HashMap<SymbolRef, Type>,
}

impl SymbolTypeTable {
    pub fn new() -> Self {
        Self {
            by_ref: HashMap::new(),
        }
    }

    pub fn get(&self, symbol_ref: &SymbolRef) -> Type {
        self.by_ref
            .get(symbol_ref)
            .cloned()
            .unwrap_or(Type::Unknown)
    }

    fn parse_stmt(
        &self,
        program_id: i64,
        stmt: &StmtIR,
        symbols: &GlobalSymbolTable,
        resolutions: &ResolutionTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Option<Type> {
        match stmt {
            StmtIR::Assign(assign_stmt) => {
                let value = &assign_stmt.value;
                let ty = match value {
                    ExprIR::Constant(ConstantIR::IntegerLit(_)) => Type::Int,
                    ExprIR::Constant(ConstantIR::FloatLit(_)) => Type::Float,
                    ExprIR::Constant(ConstantIR::BooleanLit(_)) => Type::Bool,
                    ExprIR::Constant(ConstantIR::StringLit(_)) => Type::String,
                    ExprIR::Constant(ConstantIR::NoneLit(_)) => Type::None,
                    ExprIR::Constant(ConstantIR::EllipsisLit(_)) => Type::Ellipsis,
                    ExprIR::Constant(ConstantIR::BytesLit(_)) => Type::Bytes,
                    ExprIR::Constant(ConstantIR::ComplexLit(_)) => Type::Complex,
                    _ => Type::Unknown, // what we cannot resolve directly gets an Unknown type, we will resolve it later.
                };

                Some(ty)
            }

            StmtIR::AnnAssign(annassign_stmt) => {
                Some(self.parse_annotation(
                    program_id,
                    annassign_stmt,
                    symbols,
                    resolutions,
                    diagnostics,
                ))
            }

            // this doesn't make sense, should just be assign and annassign ..? what else could be Unknown, nothing
            _ => {
                None
            }
        }
    }

    fn parse_annotation(
        &self,
        program_id: i64,
        annassign_stmt: &AnnAssignIR,
        symbols: &GlobalSymbolTable,
        resolutions: &ResolutionTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Type {
        // only bindings with annotations arrive here
        let annotation = &annassign_stmt.annotation;

        let root: Type = match annotation.head.root.as_str() {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            "str" => Type::String,
            "None" => Type::None,
            _ => self.resolve_annotation_path(
                // check global symbol table to get the ref, ie is this torch, numpy, Local, true Unknown?
                annotation.head.root.as_str(),
                annotation.head.attrs.as_slice(),
                program_id,
                symbols,
                resolutions,
            ),
        };

        root
    }

    fn resolve_annotation_path(
        &self,
        root: &str,
        attrs: &[String],
        program_id: i64,
        symbols: &GlobalSymbolTable,
        resolutions: &ResolutionTable,
    ) -> Type {
        let symbol_ref = match symbols.lookup(program_id, root) {
            Some(symbol_ref) => symbol_ref,
            None => return Type::Unknown,
        };

        let target = match resolutions.imports.get(symbol_ref) {
            Some(target) => target,
            None => return Type::Unknown,
        };

        match target {
            ResolvedTarget::Local(local_ref) if attrs.is_empty() => {
                self.by_ref.get(local_ref).cloned().unwrap_or(Type::Unknown)
            }

            ResolvedTarget::External { module, name } => {
                self.resolve_external_path(module, name, attrs)
            }

            _ => Type::Unknown,
        }
    }

    fn resolve_external_path(&self, module: &str, imported_name: &str, attrs: &[String]) -> Type {
        let mut path = Vec::new();

        // `import torch`: the imported name represents the module itself.
        if imported_name != module {
            path.push(imported_name);
        }

        path.extend(attrs.iter().map(String::as_str));

        match (module, path.as_slice()) {
            ("torch", ["Tensor"]) => Type::Tensor(TensorTypeState::Unresolved),

            ("torch", ["Size"]) => Type::Dim(DimType::Unknown),

            ("torch", ["nn", "Parameter"]) => Type::Tensor(TensorTypeState::Unresolved),

            _ => Type::Unknown,
        }
    }
}

//     pub fn build(
//         &mut self,
//         programs: &ProgramTable,
//         symbols: &GlobalSymbolTable,
//         resolutions: &ResolutionTable,
//     ) -> Result<(), Vec<Diagnostic>> {
//         let mut diagnostics = Vec::new();

//         for (&program_id, program) in &programs.by_id {
//             for decl in &program.decls {
//                 let symbol_ref = SymbolRef {
//                     program_id,
//                     symbol_id: decl.symbol_id(),
//                 };

//                 let symbol_type = match decl {
//                     DeclIR::Binding(binding) => self.parse_binding(
//                         program_id,
//                         &binding,
//                         symbols,
//                         resolutions,
//                         &mut diagnostics,
//                     ),

//                     DeclIR::Function(_) => Type::Callable(CallableType {
//                         params: Vec::new(),
//                         return_type: Box::new(Type::Unknown),
//                     }),

//                     DeclIR::Class(_) => Type::Class(ClassType { symbol: symbol_ref }),
//                 };

//                 self.by_ref.insert(symbol_ref, symbol_type);
//             }
//         }

//         if diagnostics.is_empty() {
//             Ok(())
//         } else {
//             Err(diagnostics)
//         }
//     }
// }
