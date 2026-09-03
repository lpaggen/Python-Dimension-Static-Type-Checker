use crate::control_flow::bindingstate::BindingState;
use crate::control_flow::bound_type::TypedBinding;
use crate::control_flow::flowstate::FlowState;
use crate::diagnostic::diagnostic::Diagnostic;
use crate::ir::expr::BinOpIR;
use crate::ir::expr::CallIR;
use crate::ir::expr::NameIR;
use crate::ir::operator;
use crate::ir::operator::Operator;
use crate::ir::stmt::AnnAssignIR;
use crate::ir::stmt::StmtIR;
use crate::linker::symbol_ref;
use std::collections::HashMap;

use crate::ir::expr_ir::ConstantIR;
use crate::ir::expr_ir::ExprIR;
use crate::linker::scope_table::GlobalSymbolTable;
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

    pub fn parse_expr(
        &self,
        expr: &ExprIR,
        program_id: i64,
        state: &mut FlowState,
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

            // can be many things, notably torch.tensor(...) 
            // so this is where we start parsing tensors, amongst other things
            ExprIR::Call(callexpr) => {
                println!("{:?}", callexpr);
                Type::Unknown
            }

            ExprIR::TupleExpr(tuple) => {
                let element_types = tuple
                    .elts
                    .iter()
                    .map(|element| self.parse_expr(
                        element, 
                        program_id, 
                        state
                    ))
                    .collect();

                Type::Tuple(element_types)
            }

            ExprIR::BinOpExpr(binop) => {
                match &binop.op {

                    // OR-union -> either A or B, is this the best way to represent it?
                    Operator::BitOr => {  // x: int | None, "|" is the operator
                        Type::Union(
                            vec![
                                self.parse_expr(&binop.left, program_id, state), 
                                self.parse_expr(&binop.right, program_id, state)
                            ],
                        )
                    },

                    Operator::Add => {
                        self.resolve_add(
                            self.parse_expr(&binop.left, program_id, state),
                            self.parse_expr(&binop.right, program_id, state),
                        )
                    },

                    Operator::Sub => todo!(),
                    Operator::Mult => todo!(),
                    Operator::MatMult => todo!(),
                    Operator::Div => todo!(),
                    Operator::FloorDiv => todo!(),
                    Operator::Mod => todo!(),
                    Operator::Pow => todo!(),
                    Operator::LShift => todo!(),
                    Operator::RShift => todo!(),
                    Operator::BitXor => todo!(),
                    Operator::BitAnd => todo!(),
                    Operator::UAdd => todo!(),
                    Operator::USub => todo!(),
                    Operator::Not => todo!(),
                    Operator::Invert => todo!(),
                    Operator::And => todo!(),
                    Operator::Or => todo!(),
                    Operator::Eq => todo!(),
                    Operator::NotEq => todo!(),
                    Operator::Lt => todo!(),
                    Operator::LtE => todo!(),
                    Operator::Gt => todo!(),
                    Operator::GtE => todo!(),
                    Operator::Is => todo!(),
                    Operator::IsNot => todo!(),
                    Operator::In => todo!(),
                    Operator::NotIn => todo!(),
                    Operator::AddAssign => todo!(),
                    Operator::SubAssign => todo!(),
                    Operator::MultAssign => todo!(),
                    Operator::MatMultAssign => todo!(),
                    Operator::DivAssign => todo!(),
                    Operator::FloorDivAssign => todo!(),
                    Operator::ModAssign => todo!(),
                    Operator::PowAssign => todo!(),
                    Operator::LShiftAssign => todo!(),
                    Operator::RShiftAssign => todo!(),
                    Operator::BitOrAssign => todo!(),
                    Operator::BitXorAssign => todo!(),
                    Operator::BitAndAssign => todo!(),
                    Operator::Walrus => todo!(),
                    Operator::Unknown(_) => todo!(),
                }
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
            // AND THEREFORE ENTIRELY DEPENDS ON FLOW STATE BEING COMPLETE!!!!!!!!!!!!!!!!!!!!

            // also this depends on the type of the annotation
            ExprIR::Name(name) => {
                let symbol_ref = match &self.symbols.lookup_by_name(
                    program_id, 
                    name.use_scope_id, 
                    &name.id
                ) {
                    Some(reference) => {
                        reference
                    },

                    None => {
                        panic!("not found symbol")
                    }
                };

                // match state.by_ref.get(symbol_ref) {
                //     Some(TypedBinding {
                //         binding: BindingState::Bound,
                //         ty,
                //     }) => ty,clone(),

                //     Some(TypedBinding {
                //         binding: BindingState::MaybeUnbound,
                //         ty,
                //     }) => {
                //         // add warning here
                //         ty.clone()
                //     }

                //     _ => Type::Unknown
                // }

                Type::Unknown
            },

            ExprIR::SliceExpr(slice) => {
                Type::Unknown
            }

            ExprIR::SubscriptExpr(subscript) => {
                Type::Unknown
            }

            ExprIR::Attribute(attribute) => {
                Type::Unknown
            }

            ExprIR::BoolOpExpr(boolean) => {
                Type::Unknown
            }

            ExprIR::UnaryOpExpr(unary) => {
                Type::Unknown
            }

            ExprIR::CompareExpr(cmp) => {
                Type::Unknown
            }

            _ => Type::Unknown,
        }
    }

    fn promote_numeric(&self, left: Type, right: Type) -> Type {
        match (&left, &right) {
            (Type::Complex, _) | (_, Type::Complex) => Type::Complex,
            (Type::Float, _) | (_, Type::Float) => Type::Float,
            (Type::Int, _) | (_, Type::Int) => Type::Int,
            (Type::Bool, Type::Bool) => Type::Int, // depending on exact operator semantics
            _ => Type::Unknown,
        }
    }

    fn resolve_add(&self, left: Type, right: Type) -> Type {
        if left.is_numeric() && right.is_numeric() {
            return self.promote_numeric(left, right);
        }

        match (left, right) {
            (Type::String, Type::String) => Type::String,

            (Type::List(a), Type::List(b)) => {
                // Type::List(self.merge_element_types(a, b))
                Type::Unknown
            }

            (Type::Tuple(a), Type::Tuple(b)) => {
                // concatenate tuple type information
                Type::Unknown
            }

            (Type::Tensor(a), Type::Tensor(b)) => {
                // self.resolve_tensor_add(a, b)
                Type::Unknown
            }

            (Type::Tensor(a), scalar) if scalar.is_numeric() => {
                // self.resolve_tensor_scalar_add(a, scalar)
                Type::Unknown
            }

            (scalar, Type::Tensor(b)) if scalar.is_numeric() => {
                // self.resolve_scalar_tensor_add(scalar, b)
                Type::Unknown
            }

            (Type::Union(items), rhs) => {
                // self.distribute_binop_over_union(Operator::Add, items, rhs)
                Type::Unknown
            }

            (lhs, Type::Union(items)) => {
                // self.distribute_binop_over_union(Operator::Add, vec![lhs], Type::Union(items))
                Type::Unknown
            }

            _ => Type::Unknown,
        }
    }

    // recall everything maps from SymbolRef to a canonical External type which has the disambiguated name
    // so these paths are always valid
    fn resolve_external_annotation(&self, module: &str, name: &str) -> Type {
        match (module, name) {
            ("torch", "Tensor") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            ("torch", "Size") => {
                Type::Dim(DimType::Unknown)
            }

            ("torch.nn", "Parameter") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            // TODO complete list as we need more and more types

            _ => Type::Unknown,
        }
    }

    fn resolve_annotation_name(&self, program_id: i64, name: &NameIR) -> Type {
        let symbol_ref = match self.symbols.global_lookup(program_id, &name.id) {
            Some(symbol_ref) => symbol_ref,
            None => return Type::Unknown,
        };

        let target = match self.resolutions.imports.get(&symbol_ref) {
            Some(target) => target,
            None => return Type::Unknown,
        };

        match target {
            ResolvedTarget::Local(local_ref) => {
                self.by_ref.get(local_ref).cloned().unwrap_or(Type::Unknown)
            }

            ResolvedTarget::External { module, name } => {
                self.resolve_external_annotation(module, name)
            }

            _ => Type::Unknown,
        }
    }

    fn parse_annotation(&self, expr: &ExprIR, program_id: i64) -> Type {
        match expr {
            ExprIR::Name(name) => match name.id.as_str() {
                "int" => Type::Int,
                "float" => Type::Float,
                "str" => Type::String,
                "bool" => Type::Bool,
                "bytes" => Type::Bytes,
                "None" => Type::None,

                _ => {
                    // resolve user-defined/imported type name
                    // example "torch" or "numpy" if external, if local something you defined, a class, a type, etc
                    self.resolve_annotation_name(program_id, name)
                }
            },

            ExprIR::BinOpExpr(binop) if matches!(binop.op, Operator::BitOr) => {
                let left = self.parse_annotation(&binop.left, program_id);
                let right = self.parse_annotation(&binop.right, program_id);

                Type::Union(vec![left, right])
            },

            ExprIR::SubscriptExpr(subscript) => {
                // list[int], tuple[str, int], Tensor[...], etc.
                Type::Unknown
            }

            ExprIR::Attribute(attribute) => {
                // torch.Tensor, typing.Optional, etc.
                Type::Unknown
            }

            // _ => self.resolve_annotation_path(root, attrs, program_id)
            _ => Type::Unknown  // TODO for now, but fix later
        }
    }

    fn types_compatible(&self, left: &Type, right: &Type) -> bool {
        false
    }

    pub fn resolve_type(
        &self,
        program_id: i64,
        stmt: &StmtIR,
        state: &mut FlowState,
    ) -> Type {
        match stmt {
            StmtIR::Assign(assign_stmt) => {
                let ty = self.parse_expr(
                    &assign_stmt.value, 
                    program_id, 
                    state
                );
                ty
            },

            StmtIR::AnnAssign(annassign_stmt) => {
                let annotation_type = self.parse_annotation(
                    &annassign_stmt.annotation, 
                    program_id, 
                );

                match &annassign_stmt.value {
                    Some(value) => {
                        let value_type = self.parse_expr(
                            value,
                            program_id, 
                            state
                        );

                        if self.types_compatible(&value_type, &annotation_type) {
                            annotation_type
                        } else {
                            // TODO + emit a diagnostic warning
                            Type::Unknown
                        }
                    },

                    None => annotation_type
                }
            },

            _ => {
                panic!()
            }
        }
    }

    // fn parse_annotation(
    //     &self,
    //     program_id: i64,
    //     annassign_stmt: &AnnAssignIR,
    // ) -> Type {
    //     // only bindings with annotations arrive here
    //     let annotation = &annassign_stmt.annotation;

    //     let root: Type = match annotation.head.root.as_str() {
    //         "int" => Type::Int,
    //         "float" => Type::Float,
    //         "bool" => Type::Bool,
    //         "str" => Type::String,
    //         "bytes" => Type::Bytes,
    //         "complex" => Type::Complex,
    //         "None" => Type::None,
    //         _ => self.resolve_annotation_path(
    //             // check global symbol table to get the ref, ie is this torch, numpy, Local, true Unknown?
    //             annotation.head.root.as_str(),
    //             annotation.head.attrs.as_slice(),
    //             program_id,
    //         ),
    //     };

    //     root
    // }

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
