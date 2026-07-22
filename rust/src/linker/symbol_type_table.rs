use std::{collections::HashMap};
use crate::diagnostic::diagnostic::Diagnostic;

use crate::diagnostic::diagnostic::DiagnosticKind;
use crate::diagnostic::diagnostic::Severity;

use crate::ir::expr_ir::ExprIR;
use crate::ir::nodes::BindingIR;
use crate::ir::nodes::binding_ir::BindingKind;
use crate::linker::global_scope_table::GlobalSymbolTable;
use crate::linker::resolution_table::ResolutionTable;
use crate::linker::resolved_target::ResolvedTarget;
use crate::types::types::CallableType;
use crate::types::types::ClassType;
use crate::types::types::TensorTypeState;
use crate::types::types::Type;
use crate::{ir::nodes::DeclIR, linker::{program_table::ProgramTable, symbol_ref::SymbolRef}};
use crate::types::types::DimType;

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
            },

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

        let root: Type = match annotation.head.root.as_str() {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            "str" => Type::String,
            "None" => Type::None,
            _ => self.resolve_annotation_path(  // check global symbol table to get the ref, ie is this torch, numpy, Local, true Unknown? 
                annotation.head.root.as_str(), 
                annotation.head.attrs.as_slice(),
                program_id, 
                symbols, 
                resolutions),
        };

        root  // safe? or allow Unknown..? 
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
            ResolvedTarget::Local(local_ref) if attrs.is_empty() => self
                .by_ref
                .get(local_ref)
                .cloned()
                .unwrap_or(Type::Unknown),

            ResolvedTarget::External { module, name } => {
                self.resolve_external_path(module, name, attrs)
            }

            _ => Type::Unknown,
        }
    }

    fn resolve_external_path(
        &self,
        module: &str,
        imported_name: &str,
        attrs: &[String],
    ) -> Type {
        let mut path = Vec::new();

        // `import torch`: the imported name represents the module itself.
        if imported_name != module {
            path.push(imported_name);
        }

        path.extend(attrs.iter().map(String::as_str));

        match (module, path.as_slice()) {
            ("torch", ["Tensor"]) => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            ("torch", ["Size"]) => {
                Type::Dim(DimType::Unknown)
            }

            ("torch", ["nn", "Parameter"]) => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            _ => Type::Unknown,
        }
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
