use crate::diagnostic::diagnostic::Diagnostic;
use crate::ir::expr::CallIR;
use crate::ir::stmt::AnnAssignIR;
use crate::ir::stmt::StmtIR;
use std::arch::naked_asm;
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
// !! Type resolver is only used in the CFG phase, we likely won't be calling build() standalone, rather just resolve stmt by stmt
pub struct TypeResolver<'ctx> {
    pub by_ref: HashMap<SymbolRef, Type>,
    pub diagnostics: Vec<Diagnostic>,

    symbols: &'ctx GlobalSymbolTable,
    resolutions: &'ctx ResolutionTable,
}

impl<'ctx> TypeResolver<'ctx> {
    pub fn new(
        symbols: &'ctx GlobalSymbolTable,
        resolutions: &'ctx ResolutionTable,
    ) -> Self {
        Self {
            by_ref: HashMap::new(),
            symbols,
            resolutions,
            diagnostics: Vec::new(),
        }
    }

    pub fn get(&self, symbol_ref: &SymbolRef) -> Type {
        self.by_ref
            .get(symbol_ref)
            .cloned()
            .unwrap_or(Type::Unknown)
    }

    // pub fn infer_call_type(&self, call: &CallIR) -> Option<Type> {
    //     match &call.func {
    //         ExprIR::Name(name) => {
    //             // should resolve to the right target, then check if it has a return type
    //             todo!()
    //         }

    //         _ => {
    //             panic!("Call to function invalid")
    //         }
    //     }
    // }
    
    fn parse_expr(
        &self,
        expr: &ExprIR,
        program_id: i64,
    ) -> Type {
        match expr {
            ExprIR::Constant(ConstantIR::IntegerLit(_)) => Type::Int,
            ExprIR::Constant(ConstantIR::FloatLit(_)) => Type::Float,
            ExprIR::Constant(ConstantIR::BooleanLit(_)) => Type::Bool,
            ExprIR::Constant(ConstantIR::StringLit(_)) => Type::String,
            ExprIR::Constant(ConstantIR::NoneLit(_)) => Type::None,
            ExprIR::Constant(ConstantIR::EllipsisLit(_)) => Type::Ellipsis,
            ExprIR::Constant(ConstantIR::BytesLit(_)) => Type::Bytes,
            ExprIR::Constant(ConstantIR::ComplexLit(_)) => Type::Complex,

            ExprIR::TupleExpr(tuple) => {
                let element_types = tuple
                    .elts
                    .iter()
                    .map(|element| self.parse_expr(element, program_id))
                    .collect();

                Type::Tuple(element_types)
            }

            // defer to later
            // ExprIR::Call(call) => {
            //     self.infer_call_type(call)
            // },

            // ExprIR::BinOpExpr(binop_expr) => {
            //     self.infer_binary_type(binop_expr.op, &binop_expr.left, &binop_expr.right)
            // },

            // might want to delegate to a later pass? we will see
            // name resolution | x: int = a <- we need to find what Type "a" is, is it declared? accessible? unbound?
            ExprIR::Name(name) => {
                // let symbol_ref = SymbolRef {
                //     program_id: ...,
                //     symbol_id: identifier.name.
                // }

                // need to find by &identifer.name somehow...
                // 1) find ref from name
                // 2) query flow_env with ref

                // self.flow_env.get(k)

                Type::Unknown
            },

            ExprIR::SliceExpr(slice) => {
                todo!()
            }

            ExprIR::SubscriptExpr(subscript) => {
                todo!()
            }

            ExprIR::Attribute(attribute) => {
                todo!()
            }

            ExprIR::BoolOpExpr(boolean) => {
                todo!()
            }

            ExprIR::UnaryOpExpr(unary) => {
                todo!()
            }

            ExprIR::CompareExpr(cmp) => {
                todo!()
            }

            _ => Type::Unknown,
        }
    }

    pub fn parse_stmt(
        &self,
        program_id: i64,
        stmt: &StmtIR,
    ) -> Option<Type> {
        match stmt {
            StmtIR::Assign(assign_stmt) => {
                let ty = self.parse_expr(&assign_stmt.value, program_id);
                Some(ty)
            }

            StmtIR::AnnAssign(annassign_stmt) => {
                Some(self.parse_annotation(
                    program_id,
                    annassign_stmt,
                ))
            }

            _ => {
                None
            }
        }
    }

    fn parse_annotation(
        &self,
        program_id: i64,
        annassign_stmt: &AnnAssignIR,
    ) -> Type {
        // only bindings with annotations arrive here
        let annotation = &annassign_stmt.annotation;

        let root: Type = match annotation.head.root.as_str() {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            "str" => Type::String,
            "bytes" => Type::Bytes,
            "complex" => Type::Complex,
            "None" => Type::None,
            _ => self.resolve_annotation_path(
                // check global symbol table to get the ref, ie is this torch, numpy, Local, true Unknown?
                annotation.head.root.as_str(),
                annotation.head.attrs.as_slice(),
                program_id,
            ),
        };

        root
    }

    fn resolve_annotation_path(
        &self,
        root: &str,
        attrs: &[String],
        program_id: i64,
    ) -> Type {
        let symbol_ref = match self.symbols.global_lookup(program_id, root) {
            Some(symbol_ref) => symbol_ref,
            None => return Type::Unknown,
        };

        let target = match self.resolutions.imports.get(&symbol_ref) {
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

    // WARNING THIS BREAKS FOR: "from torch.nn import Parameter" for example
    // TODO fix, should be some invariant which makes this work neatly
    fn resolve_external_path(&self, module: &str, imported_name: &str, attrs: &[String]) -> Type {
        let mut path = Vec::new();

        // "import torch": the imported name represents the module itself
        // ? recall how this works in general
        // i think through ResolvedSymbol all imports map back to their canonical name
        if imported_name != module {
            path.push(imported_name);
        }

        path.extend(attrs.iter().map(String::as_str));

        // so the following could always hold normally
        // TODO check if it works for "from torch import Tensor", would we still get "torch" as root ? 
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
