use crate::{
    diagnostic::diagnostic::Diagnostic, ir::{expr_ir::ExprIR, nodes::{AnnotationIR, BindingIR, CallExprIR, ClassIR, DeclIR, NoneIR, binding_ir::BindingKind}, operator::Operator}, linker::{
        global_scope_table::GlobalSymbolTable, program_table::ProgramTable, resolution_table::ResolutionTable, symbol_ref::SymbolRef, symbol_type_table::SymbolTypeTable
    }, pb::expr_ir::Kind::Tuple, types::types::{TensorType, Type::{
            self,
            None
    }}
};

pub struct TypeResolver<'a> {
    pub diagnostics: &'a mut Vec<Diagnostic>,
    symbol_types: &'a mut SymbolTypeTable,  // we udpate it as we go
}

impl<'a> TypeResolver<'a> {

    fn new(symbol_types: &'a mut SymbolTypeTable, diagnostics: &'a mut Vec<Diagnostic>,) -> Self {
        Self {
            symbol_types,
            diagnostics,
        }
    }

    // walk self.scopes (TODO add this as borrow in struct)
    fn resolve_name(&self, mut scope: i64, name: &str) -> Option<SymbolRef> {
        todo!()
    }

    /// checks whether LHS and RHS are compatible and can be cast to another type according to Python's type casting rules
    fn resolve_binary_types(&self, op: Operator, left: Type, right: Type) -> Type {
        todo!()
    }

    fn resolve_call(&self, call: &CallExprIR, scope_id: i64) -> Type {
        todo!()
    }

    // TODO give it ScopeID too, this is extremely important
    fn resolve_expr(&self, expr: &ExprIR, scope_id: i64) -> Type {
        match expr {
            ExprIR::IntegerExpr(_) => Type::Int,
            ExprIR::FloatExpr(_) => Type::Float,
            ExprIR::BoolExpr(_) => Type::Bool,
            ExprIR::StringExpr(_) => Type::String,
            ExprIR::NoneExpr(_) => Type::None,
            ExprIR::BoolExpr(_) => Type::Bool,

            ExprIR::ListExpr(list) => {
                let element_types = list.elements
                    .iter()
                    .map(|element| self.resolve_expr(element, scope_id))
                    .collect();

                Type::List(element_types)
            }

            // may want to check on the immutability? 
            ExprIR::TupleExpr(tuple) => {
                let element_types = tuple.elements
                    .iter()
                    .map(|element| self.resolve_expr(element, scope_id))
                    .collect();

                Type::Tuple(element_types)
            }

            ExprIR::SliceExpr(SliceIR) => {
                todo!()
            }

            ExprIR::SubscriptExpr(SubscriptIR) => {
                todo!()
            }

            ExprIR::AttributeExpr(AttributeExprIR) => {
                todo!()
            }

            ExprIR::BinOpExpr(BinOpIR) => {
                todo!()
            }

            ExprIR::BoolOpExpr(BoolOpIR) => {
                todo!()
            }

            ExprIR::UnaryOpExpr(UnaryOpIR) => {
                todo!()
            }

            ExprIR::CompareExpr(CompareIR) => {
                todo!()
            }

            ExprIR::CallExpr(CallExprIR) => {
                todo!()
            }

            // variables, example -> x: int = y (i want to check if y is in a parent scope)
            ExprIR::IdentifierExpr(name) => {
                let Some(symbol) = self.resolve_name(scope_id, &name.name) else {
                    return Type::Unknown;
                };

                self.symbol_types.by_ref
                    .get(&symbol)
                    .cloned()
                    .unwrap_or(Type::Unknown)
            }

            ExprIR::CallExpr(call) => self.resolve_call(call, scope_id),

            ExprIR::BinOpExpr(binary) => {
                let lhs = self.resolve_expr(&binary.left, scope_id);
                let rhs = self.resolve_expr(&binary.right, scope_id);

                self.resolve_binary_types(binary.op, lhs, rhs)
            }

            _ => Type::Unknown,
        }
    }

    pub fn resolve_annassign(&self, binding_ir: &BindingIR, program_id: i64) -> Type {

        let symbol_ref = SymbolRef {
            program_id,
            symbol_id: binding_ir.id
        };

        // get the existing type, resolved via annotation only, can be Unknown otherwise
        let annotation_type: &Type = self.symbol_types.by_ref.get(&symbol_ref).unwrap_or(&Type::Unknown);

        let value = match &binding_ir.value {
            Some(value) => self.resolve_expr(value, binding_ir.scope_id),
            _other_none => annotation_type.clone(),
        };

        Type::Unknown
    }

    /// resolves types on RHS, example -> x: int = 5, match "int" and "5". or -> x = 5 -> go from Type::Unknown to Type::Int
    pub fn resolve_types(&self, resolutions: &ResolutionTable, programs: &ProgramTable) {
        // TODO need to resolve the actual RHS of each decl
        // meaning we need access to scopes too, since RHS may be in specific local scopes etc, can only resolve if outer scope <= own scope
        for (&program_id, program) in &programs.by_id {
            for decl in &program.decls {
                let final_type = match decl {
                    DeclIR::Binding(binding_ir) => {
                        match &binding_ir.kind {
                            BindingKind::Assign => {
                                todo!()
                            }

                            BindingKind::AnnAssign => {
                                self.resolve_annassign(binding_ir, binding_ir.scope_id)
                            },

                            BindingKind::Unknown => {
                                todo!()
                            },
                        }
                    }

                    DeclIR::Class(class_ir) => {
                        todo!()
                    }

                    DeclIR::Function(function_ir) => {
                        todo!()
                    }
                };
            }
        }
    }
}
