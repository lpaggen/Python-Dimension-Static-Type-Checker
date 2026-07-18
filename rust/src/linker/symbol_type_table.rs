use std::{collections::HashMap};
use crate::diagnostic::diagnostic::Diagnostic;

use crate::diagnostic::diagnostic::DiagnosticKind;
use crate::diagnostic::diagnostic::Severity;

use crate::ir::expr_ir::ExprIR;
use crate::ir::nodes::BindingIR;
use crate::ir::nodes::binding_ir::BindingKind;
use crate::types::types::CallableType;
use crate::types::types::ClassType;
use crate::types::types::Type;
use crate::{ir::nodes::DeclIR, linker::{program_table::ProgramTable, symbol_ref::SymbolRef}};



pub struct SymbolTypeTable {
    pub by_ref: HashMap<SymbolRef, Type>,
}

impl SymbolTypeTable {
    pub fn new() -> Self {
        Self { 
            by_ref: HashMap::new(), 
        }
    }

    fn parse_binding(&self, binding: &BindingIR, diagnostics: &mut Vec<Diagnostic>) -> Type {
        match &binding.kind {
            BindingKind::Assign => {
                let value = match &binding.value {
                    Some(value) => &**value,
                    None => {
                        diagnostics.push(Diagnostic {
                            severity: Severity::ERROR,
                            span: binding.span.clone(),
                            kind: DiagnosticKind::MissingBindingValue,
                            message: "frontend found no binding value".into(),
                        });
                        return Type::Unknown;
                    }
                };
                match value {
                    ExprIR::IntegerExpr(_) => Type::Int,
                    ExprIR::FloatExpr(_) => Type::Float,
                    ExprIR::BoolExpr(_) => Type::Bool,
                    ExprIR::StringExpr(_) => Type::String,
                    ExprIR::NoneExpr(_) => Type::None,
                    _ => Type::Unknown,  // what we cannot resolve directly gets an Unknown type, we will resolve it later. 
                }
            },

            BindingKind::AnnAssign => {
                Type::Unknown
            },  // type is not known yet, since we don't parse annotation here yet

            BindingKind::Unknown => {  // safe to deprecate ? makes no sense to return Unknown at any point after parsing
                diagnostics.push(Diagnostic {
                    severity: Severity::ERROR,
                    span: binding.span.clone(),
                    kind: DiagnosticKind::InvalidBindingIR,
                    message: "binding kind was not classified by the frontend".into(),
                });
                Type::Unknown
            }
        }
    }

    pub fn build(&mut self, programs: &ProgramTable) -> Result<SymbolTypeTable, Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        let mut by_ref = HashMap::new();

        for (&program_id, program) in &programs.by_id {
            for decl in &program.decls {
                let symbol_ref = SymbolRef {
                    program_id,
                    symbol_id: decl.symbol_id(),
                };

                let symbol_type = match decl {
                    DeclIR::Binding(binding) => {
                        self.parse_binding(binding, &mut diagnostics)
                    }

                    DeclIR::Function(_) => {
                        Type::Callable(CallableType {
                            params: Vec::new(),
                            return_type: Box::new(Type::Unknown),
                        })
                    }

                    DeclIR::Class(_) => {
                        Type::Class(ClassType {
                            symbol: symbol_ref,
                        })
                    }
                };

                by_ref.insert(symbol_ref, symbol_type);
            }
        }

        if diagnostics.is_empty() {
            Ok(SymbolTypeTable { 
                by_ref 
            })
        } else {
            Err(diagnostics)
        }
    }
}
