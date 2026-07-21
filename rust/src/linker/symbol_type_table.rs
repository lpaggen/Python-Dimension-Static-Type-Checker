use std::{collections::HashMap};
use crate::diagnostic::diagnostic::Diagnostic;

use crate::diagnostic::diagnostic::DiagnosticKind;
use crate::diagnostic::diagnostic::Severity;

use crate::ir::expr_ir::ExprIR;
use crate::ir::nodes::BindingIR;
use crate::ir::nodes::binding_ir::BindingKind;
use crate::linker::global_scope_table::GlobalSymbolTable;
use crate::linker::resolution_table::ResolutionTable;
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

    fn parse_binding(&self,
        program_id: i64,
        binding: &BindingIR,
        symbols: &GlobalSymbolTable,
        resolutions: &ResolutionTable,
        diagnostics: &mut Vec<Diagnostic>
    ) -> Type {
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
                self.parse_annotation(program_id, &binding, symbols, resolutions, diagnostics)
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

    fn parse_annotation(
        &self,
        program_id: i64,
        binding: &BindingIR,
        symbols: &GlobalSymbolTable,
        resolutions: &ResolutionTable,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Type  {  // only bindings with annotations arrive here
        let annotation = match &binding.annotation {
            Some(annotation) => annotation,
            None => {
                diagnostics.push(Diagnostic {
                    severity: Severity::ERROR,
                    span: binding.span.clone(),
                    kind: DiagnosticKind::InvalidBindingIR,
                    message: "annotated binding has no annotation".into(),
                });
                return Type::Unknown;
            }
        };

        println!("{:?}", annotation);

        let root: Type = match annotation.head.root.as_str() {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            "str" => Type::String,
            "None" => Type::None,
            _ => Type::Unknown,
        };

        Type::Unknown
    }

    pub fn build(
        &mut self,
        programs: &ProgramTable,
        symbols: &GlobalSymbolTable,
        resolutions: &ResolutionTable,
    ) -> Result<(), Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        for (&program_id, program) in &programs.by_id {
            for decl in &program.decls {
                let symbol_ref = SymbolRef {
                    program_id,
                    symbol_id: decl.symbol_id(),
                };

                let symbol_type = match decl {
                    DeclIR::Binding(binding) => {
                        self.parse_binding(program_id, binding, symbols, resolutions, &mut diagnostics)
                    }

                    DeclIR::Function(_) => Type::Callable(CallableType {
                        params: Vec::new(),
                        return_type: Box::new(Type::Unknown),
                    }),

                    DeclIR::Class(_) => Type::Class(ClassType {
                        symbol: symbol_ref,
                    }),
                };

                self.by_ref.insert(symbol_ref, symbol_type);
            }
        }

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
}
