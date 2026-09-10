use rayon::vec;

use crate::control_flow::bindingstate::BindingState;
use crate::control_flow::bound_type::TypedBinding;
use crate::control_flow::flowstate::FlowState;
use crate::diagnostic::diagnostic::Diagnostic;
use crate::diagnostic::diagnostic::DiagnosticKind;
use crate::diagnostic::diagnostic::Severity;
use crate::ir::expr::AttributeIR;
use crate::ir::expr::BinOpIR;
use crate::ir::expr::CallIR;
use crate::ir::expr::KeywordIR;
use crate::ir::expr::ListIR;
use crate::ir::expr::NameIR;
use crate::ir::expr::TupleIR;
use crate::ir::operator::Operator;
use crate::ir::stmt::AnnAssignIR;
use crate::ir::stmt::StmtIR;
use crate::type_resolver::library::KnownFunction;
use crate::type_resolver::library::KnownLibrary;
use crate::type_resolver::library::KnownLibrary::PyTorch;
use crate::type_resolver::library::Library;
use crate::type_resolver::library::NumPyFunction;
use crate::type_resolver::library::ResolvedAttributePath;
use crate::type_resolver::library::TorchFunction;

use crate::ir::expr_ir::ConstantIR;
use crate::ir::expr_ir::ExprIR;
use crate::linker::scope_table::GlobalSymbolTable;
use crate::linker::resolution_table::ResolutionTable;
use crate::linker::resolved_target::ResolvedTarget;
use crate::types::types::DType;
use crate::types::types::DimType;
use crate::types::types::TensorType;
use crate::types::types::TensorTypeState;
use crate::types::types::Type;
use crate::{
    linker::{symbol_ref::SymbolRef},
};

// probably won't need 'by_ref' since this struct might just be owned by the CFG pass in a later build
// !! Type resolver is only used in the CFG phase, we likely won't be calling build() standalone, rather just resolve stmt by stmt
pub struct TypeResolver<'ctx> {
    // pub by_ref: HashMap<SymbolRef, Type>,
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
            // by_ref: HashMap::new(),
            symbols,
            resolutions,
            diagnostics: Vec::new(),
        }
    }

    // pub fn get(&self, symbol_ref: &SymbolRef) -> Type {
    //     self.by_ref
    //         .get(symbol_ref)
    //         .cloned()
    //         .unwrap_or(Type::Unknown)
    // }

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

    // similar to resolve_external_annotation, not the same return type
    fn resolve_known_function(
        &self,
        root: KnownLibrary,
        attrs: &[String],
    ) -> Option<KnownFunction> {
        match (root, attrs) {
            // PyTorch
            (KnownLibrary::PyTorch, [name])
                if name == "tensor" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Tensor))
            }

            (KnownLibrary::PyTorch, [name])
                if name == "zeros" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Zeros))
            }

            (KnownLibrary::PyTorch, [name])
                if name == "ones" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Ones))
            }

            (KnownLibrary::PyTorch, [name])
                if name == "empty" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Empty))
            }

            (KnownLibrary::PyTorch, [name])
                if name == "arange" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Arange))
            }

            (KnownLibrary::PyTorch, [name])
                if name == "reshape" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Reshape))
            }

            (KnownLibrary::PyTorch, [name])
                if name == "cat" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Cat))
            }

            (KnownLibrary::PyTorch, [name])
                if name == "stack" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Stack))
            }

            (KnownLibrary::PyTorch, [name])
                if name == "matmul" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Matmul))
            }

            // (KnownLibrary::PyTorch, [name])
            //     if name == "mm" =>
            // {
            //     Some(KnownFunction::Torch(TorchFunction::MatrixMatrix))
            // }

            (KnownLibrary::PyTorch, [nn, functional, relu])
                if nn == "nn"
                    && functional == "functional"
                    && relu == "relu" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Relu))
            }

            // NumPy
            (KnownLibrary::NumPy, [name])
                if name == "array" =>
            {
                Some(KnownFunction::NumPy(NumPyFunction::Array))
            }

            (KnownLibrary::NumPy, [name])
                if name == "zeros" =>
            {
                Some(KnownFunction::NumPy(NumPyFunction::Zeros))
            }

            (KnownLibrary::NumPy, [name])
                if name == "ones" =>
            {
                Some(KnownFunction::NumPy(NumPyFunction::Ones))
            }

            (KnownLibrary::NumPy, [name])
                if name == "empty" =>
            {
                Some(KnownFunction::NumPy(NumPyFunction::Empty))
            }

            (KnownLibrary::NumPy, [name])
                if name == "arange" =>
            {
                Some(KnownFunction::NumPy(NumPyFunction::Arange))
            }

            (KnownLibrary::NumPy, [name])
                if name == "reshape" =>
            {
                Some(KnownFunction::NumPy(NumPyFunction::Reshape))
            }

            (KnownLibrary::NumPy, [name])
                if name == "concatenate" =>
            {
                Some(KnownFunction::NumPy(NumPyFunction::Concatenate))
            }

            (KnownLibrary::NumPy, [name])
                if name == "stack" =>
            {
                Some(KnownFunction::NumPy(NumPyFunction::Stack))
            }

            (KnownLibrary::NumPy, [name])
                if name == "matmul" =>
            {
                Some(KnownFunction::NumPy(NumPyFunction::Matmul))
            }

            _ => None,
        }
    }

    fn resolve_name_root(
        &self,
        name: &NameIR,
        program_id: i64,
    ) -> Option<KnownLibrary> {
        let symbol_ref = self
            .symbols
            .lookup_by_name(program_id, name.use_scope_id, &name.id)?;

        let target = self
            .resolutions
            .imports
            .get(&symbol_ref)?;

        match target {
            ResolvedTarget::External { module, name: _ } => {
                KnownLibrary::from_str(module)
            }

            ResolvedTarget::Local(_) => None,
        }
    }

    fn resolve_attribute(
        &self,
        attr: &AttributeIR,
        program_id: i64,
    ) -> Option<ResolvedAttributePath> {
        match &*attr.value {
            ExprIR::Name(name) => {
                Some(ResolvedAttributePath {
                    root: self.resolve_name_root(name, program_id)?,
                    attrs: vec![attr.attr.clone()],
                })
            }

            ExprIR::Attribute(inner) => {
                let mut path = self
                    .resolve_attribute(inner, program_id)?;

                path
                    .attrs
                    .push(
                        attr
                        .attr
                        .clone()
                    );

                Some(path)
            }

            _ => None,
        }
    }

    fn infer_tensor_list(&mut self, expr: &ExprIR) -> Vec<DimType> {
        match expr {
            ExprIR::ListExpr(list) => {
                let len = list.elts.len().try_into().unwrap();

                if len == 0 {
                    return vec![DimType::Known(0)];
                }

                let first_shape = self.infer_tensor_list(&list.elts[0]);

                for elt in &list.elts[1..] {
                    let shape = self.infer_tensor_list(elt);

                    if shape != first_shape {
                        self.diagnostics.push(
                            Diagnostic {
                                severity: Severity::ERROR,
                                span: list.span.clone(),
                                kind: DiagnosticKind::ShapeError,
                                message: "tensor data has inconsistent nested dimensions".to_string(),
                            }
                        );
                        return vec![];
                    }
                }

                let mut shape = vec![DimType::Known(len)];
                shape.extend(first_shape);
                shape
            }

            ExprIR::Constant(_) => {
                vec![]
            }

            _ => {
                vec![]
            }
        }
    }

    fn infer_container_element_types(&mut self, expr: &ExprIR, program_id: i64) -> DType {
        match expr {
            ExprIR::ListExpr(list_expr) => {
                let mut element_types = Vec::new();
                
                for element in &list_expr.elts {
                    if let Some(element_type) = self.infer_tensor_data(element, program_id) {
                        element_types.push(element_type.dtype);
                    }
                }
                
                self.resolve_common_dtype(&element_types)
            }
            ExprIR::TupleExpr(tuple_expr) => {
                let mut element_types = Vec::new();
                
                for element in &tuple_expr.elts {
                    if let Some(element_type) = self.infer_tensor_data(element, program_id) {
                        element_types.push(element_type.dtype);
                    }
                }

                self.resolve_common_dtype(&element_types)
            }
            _ => DType::Unknown
        }
    }

    // follow PyTorch numeric promotion rules
    fn resolve_common_dtype(&self, element_types: &[DType]) -> DType {
        println!("elements: {:?}", element_types);

        let result = if element_types.is_empty() {
            DType::Unknown
        } else if element_types.contains(&DType::Float64) {
            DType::Float64
        } else if element_types.contains(&DType::Float32) {
            DType::Float32
        } else if element_types.contains(&DType::Int64) {
            DType::Int64
        } else if element_types.contains(&DType::Int32) {
            DType::Int32
        } else {
            DType::Unknown
        };

        result
    }


    fn infer_tensor_data(&mut self, expr: &ExprIR, program_id: i64) -> Option<TensorType> {
        let default_dtype = DType::Unknown;
        match expr {
            ExprIR::Constant(ConstantIR::IntegerLit(_)) => {
                Some(TensorType { 
                    shape: vec![],
                    dtype: DType::Int64  // TODO double check if this is the right default
                })
            }
        
            ExprIR::Constant(ConstantIR::FloatLit(_)) => {
                Some(TensorType { 
                    shape: vec![], 
                    dtype: DType::Float64
                })
            }

            // torch.tensor[[3, 4, 5]] etc
            ExprIR::ListExpr(_) => {
                Some(TensorType { 
                    shape: self.infer_tensor_list(expr), 
                    dtype: self.infer_container_element_types(expr, program_id),
                })
            },

            // should be same logic as the list ? TODO double check
            ExprIR::TupleExpr(_) => {
                Some(TensorType { 
                    shape: self.infer_tensor_list(expr), 
                    dtype: self.infer_container_element_types(expr, program_id),
                })
            }

            _ => {
                None
            }
        }
    }

    // support both bare name and torch.whatever
    fn infer_tensor_dtype(
        &self,
        kw: &KeywordIR,
        program_id: i64,
    ) -> DType {
        if kw.arg.as_deref() != Some("dtype") {
            return DType::Unknown;
        }

        match &*kw.value {
            // e.g. dtype=float32
            ExprIR::Name(name) => {
                match name.id.as_str() {
                    "bool" => DType::Bool,

                    "uint8" => DType::UInt8,

                    "int8" => DType::Int8,
                    "int16" => DType::Int16,
                    "int32" => DType::Int32,
                    "int64" => DType::Int64,

                    "float16" => DType::Float16,
                    "float32" => DType::Float32,
                    "float64" => DType::Float64,

                    "complex64" => DType::Complex64,
                    "complex128" => DType::Complex128,

                    _ => DType::Unknown,
                }
            }

            // e.g. dtype=torch.float32
            ExprIR::Attribute(attr) => {
                let Some(path) = self.resolve_attribute(attr, program_id) else {
                    return DType::Unknown;
                };

                match (path.root, path.attrs.as_slice()) {
                    (KnownLibrary::PyTorch, [name]) => {
                        match name.as_str() {
                            "bool" => DType::Bool,

                            "uint8" => DType::UInt8,

                            "int8" => DType::Int8,
                            "int16" => DType::Int16,
                            "int32" => DType::Int32,
                            "int64" => DType::Int64,

                            "float16" => DType::Float16,
                            "float32" => DType::Float32,
                            "float64" => DType::Float64,

                            "complex64" => DType::Complex64,
                            "complex128" => DType::Complex128,

                            _ => DType::Unknown,
                        }
                    }

                    _ => DType::Unknown,
                }
            }

            _ => DType::Unknown,
        }
    }

    // get more information about the tensors, their dtype, their dimensions etc
    fn infer_torch_tensor(&mut self,
        call: &CallIR, 
        program_id: i64
    ) -> Type {
        // pytorch tensor can look like: torch.tensor(3), torch.tensor([...]), need to parse possible variants
        let Some(data_arg) = call.args.first() else {
            self.diagnostics.push(
                Diagnostic {
                    severity: Severity::ERROR,
                    span: call.span.clone(),
                    kind: DiagnosticKind::TypeError,
                    message: "torch.tensor requires a data argument".to_string(),
                }
            );
            return Type::Unknown
        };

        let Some(mut info) = self.infer_tensor_data(data_arg, program_id) else {
            return Type::Tensor(TensorTypeState::Unresolved)
        };

        let dtype = match call
            .keywords
            .iter()
            .find(|kw| kw.arg.as_deref() == Some("dtype")) {
                Some(kw) => self.infer_tensor_dtype(kw, program_id),
                None => info.dtype
            };

        // maybe add some check for int and float found not good etc? 
        // will see if it makes sense to do that

        info.dtype = dtype;

        Type::Tensor(TensorTypeState::Resolved(info))

        // resolve dtype, find argument "dtype" and resolve if exists else unknown dtype (? double check)
    }

    fn require_dims_equal(
        &self,
        a: &DimType,
        b: &DimType,
        state: &mut FlowState,
    ) -> bool {
        match (a, b) {
            (DimType::Known(a), DimType::Known(b)) => a == b,

            (DimType::Known(a), DimType::Symbol(b)) => {
                state.constraints.push(b.eq(&z3::ast::Int::from_i64(*a)));
                true
            }

            (DimType::Symbol(a), DimType::Known(b)) => {
                state.constraints.push(a.eq(&z3::ast::Int::from_i64(*b)));
                true
            }

            (DimType::Symbol(a), DimType::Symbol(b)) => {
                state.constraints.push(a.eq(b));
                true
            }

            _ => false,
        }
    }

    fn require_matmul_dtype(
        &self,
        lhs: &DType,
        rhs: &DType,
    ) -> Option<DType> {
        match (lhs, rhs) {
            (DType::Unknown, _) | (_, DType::Unknown) => {
                None
            }

            (a, b) if a == b => {
                Some(a.clone())
            }

            _ => {
                // incompatible matmul operand dtypes
                None
            }
        }
    }

    fn infer_torch_matmul_type_tensor_to_tensor(&self, ) {
        todo!()
    }

    fn infer_torch_matmul(&mut self, call: &CallIR, program_id: i64, state: &mut FlowState) -> Type {
        let Some(first_arg) = call.args.get(0) else {
            return Type::Unknown
        };

        let type_first = self.parse_expr(first_arg, program_id, state);

        let Some(second_arg) = call.args.get(1) else {
            return Type::Unknown;
        };

        let type_second = self.parse_expr(second_arg, program_id, state);

        println!("{type_first:?}");
        println!("{type_second:?}");

        match (type_first, type_second) {
            (   // two fully resolved tensors (? unresolved should not even be passed here ? TODO double check)
                Type::Tensor(TensorTypeState::Resolved(TensorType {
                    shape: shape_a,
                    dtype: dtype_a,
                })),
                Type::Tensor(TensorTypeState::Resolved(TensorType {
                    shape: shape_b,
                    dtype: dtype_b,
                })),
            ) => {
                let result_shape = if self.require_dims_equal(
                    &shape_a[1], 
                    &shape_b[0], 
                    state
                ) {
                    vec![
                    shape_a[0].clone(),
                    shape_b[1].clone(),
                    ]
                } else {
                    self.diagnostics.push(
                        Diagnostic {
                            severity: Severity::ERROR,
                            span: call.span.clone(),
                            kind: DiagnosticKind::ShapeError,
                            message: format!(
                                "matmul dimensions are incompatible: {:?} and {:?}",
                                shape_a[1], shape_b[0]
                            ),
                        }
                    );
                    return Type::Unknown;
                };

                let Some(result_dtype) = self.require_matmul_dtype(&dtype_a, &dtype_b) else {
                    return Type::Unknown;
                };

                Type::Tensor(TensorTypeState::Resolved(TensorType {
                    shape: result_shape, 
                    dtype: result_dtype,
                }))
            },

            (
                Type::Union(uniontype),
                Type::Tensor(TensorTypeState::Resolved(TensorType {
                    shape: shape_b,
                    dtype: dtype_b,
                })),
            ) => {  // we should make recursive calls to handle unions? unsure how? TODO
                println!("union with first type union second type is a tensor");
                todo!()
            },

            (
                Type::Tensor(TensorTypeState::Resolved(TensorType {
                    shape: shape_a,
                    dtype: dtype_a,
                })),
                Type::Union(uniontype),
            ) => {  // we should make recursive calls to handle unions? unsure how? TODO
                println!("union with second type union first type is a tensor");
                todo!()
            },

            _ => {
                Type::Unknown
            }
        }    
    }

    pub fn parse_expr(
        &mut self,
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
            ExprIR::Call(call) => {
                match &*call.func {
                    ExprIR::Name(name) => {
                        // foo()
                        Type::Unknown
                    }

                    ExprIR::Attribute(attr) => {
                        // ex torch.attribute... <- recursive type
                        let Some(path) = self.resolve_attribute(attr, program_id) else {
                            return Type::Unknown
                        };

                        let Some(known_function) = self.resolve_known_function(path.root, &path.attrs) else {
                            return Type::Unknown;
                        };

                        match known_function {
                            KnownFunction::Torch(TorchFunction::Tensor) => {
                                // infer torch.tensor(...)
                                self.infer_torch_tensor(call, program_id)
                            }

                            KnownFunction::Torch(TorchFunction::Matmul) => {
                                // infer torch.matmul(...)
                                self.infer_torch_matmul(call, program_id, state)
                            }

                            // add the rest when happy with the basic examples
                            _ => Type::Unknown,
                        }
                    }

                    ExprIR::SubscriptExpr(subscript) => {
                        // handlers[i]()
                        Type::Unknown
                    }

                    ExprIR::Call(inner_call) => {
                        // factory()()
                        Type::Unknown
                    }

                    ExprIR::LambdaExpr(lambda) => {
                        // (lambda x: x)(1)
                        Type::Unknown
                    }

                    ExprIR::IfExp(ifexp) => {
                        // (a if cond else b)()
                        Type::Unknown
                    }

                    _ => {
                        // valid expression, but not handled yet
                        Type::Unknown
                    }
                }
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
                        let left = self.parse_expr(&binop.left, program_id, state);
                        let right = self.parse_expr(&binop.right, program_id, state);
                        self.resolve_add(left, right)
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

            // name resolution | x: int = a <- we need to find what Type "a" is, is it declared? accessible? unbound?
            // we'll update as we query the states
            ExprIR::Name(name) => {
                match self.symbols.lookup_by_name(
                    program_id,
                    name.use_scope_id,
                    &name.id,
                ) {
                    Some(reference) => {
                        match state.by_ref.get(&reference) {
                            Some(TypedBinding {
                                binding: BindingState::Bound,
                                ty,
                            }) => ty.clone(),

                            Some(TypedBinding {
                                binding: BindingState::MaybeUnbound,
                                ty,
                            }) => {
                                // add warning here
                                ty.clone()
                            }

                            _ => Type::Unknown,
                        }
                    }

                    None => Type::Unknown,
                }
            }

            ExprIR::SliceExpr(slice) => {
                Type::Unknown
            }

            ExprIR::SubscriptExpr(subscript) => {
                Type::Unknown
            }

            ExprIR::Attribute(attribute) => {
                println!("{attribute:?}");
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
            (Type::String, Type::String) => {
                Type::String
            },

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
    // TODO merge logic with Attribute parsing + fix because this can't handle longer attributes
    fn resolve_external_annotation(&self, module: &str, name: &str) -> Type {
        match (module, name) {
            // PyTorch
            ("torch", "Tensor") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            ("torch", "Size") => {
                Type::Dim(DimType::Unknown)
            }

            ("torch.nn", "Parameter") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            // NumPy
            ("numpy", "ndarray") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            // JAX
            ("jax", "Array") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            ("jax.numpy", "ndarray") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            // TensorFlow
            ("tensorflow", "Tensor") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            ("tensorflow", "Variable") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            ("tensorflow", "SparseTensor") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

            ("tensorflow", "RaggedTensor") => {
                Type::Tensor(TensorTypeState::Unresolved)
            }

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
                // self.by_ref.get(local_ref).cloned().unwrap_or(Type::Unknown)
                Type::Unknown
                // TODO fix, we are missing a bit of information here
            }

            ResolvedTarget::External { 
                module, 
                name 
            } => {
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

    // needed ?
    // fn types_compatible(&self, left: &Type, right: &Type) -> bool {
    //     false
    // }

    pub fn resolve_type(
        &mut self,
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

                        // force annotation == actual ? -> too strict, tensor(unknown) can be ok for a tensor with declared dims
                        // TODO fix in later build
                        if value_type == annotation_type {
                            annotation_type
                        } else {
                            // TODO + emit a diagnostic warning
                            self.diagnostics.push(
                                Diagnostic { 
                                    severity: Severity::ERROR, 
                                    span: annassign_stmt.span.clone(), 
                                    kind: DiagnosticKind::MismatchedAnnotationType, 
                                    message: format!("annotation {:?} does not match value type {:?}", annotation_type, value_type),
                                });
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
}
