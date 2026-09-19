use std::cell::RefCell;
use std::rc::Rc;

use z3::ast::Ast;

pub type ResolveResult = Result<Type, FunctionAnalysisRequest>;

use crate::control_flow::bindingstate::BindingState;
use crate::control_flow::bound_type::TypedBinding;
use crate::control_flow::call_binding::CallBinding;
use crate::control_flow::cfg_analysis_engine::function_contract_key::FunctionSpecializationKey;
use crate::control_flow::cfg_analysis_engine::functioncontract_table::FunctionContractTable;
use crate::control_flow::flowstate::FlowState;
use crate::control_flow::function_analysis_request::FunctionAnalysisRequest;
use crate::control_flow::function_contract::FunctionContract;
use crate::diagnostic::diagnostic::Diagnostic;
use crate::diagnostic::diagnostic::DiagnosticKind;
use crate::diagnostic::diagnostic::Severity;
use crate::ir::expr::AttributeIR;
use crate::ir::expr::CallIR;
use crate::ir::expr::KeywordIR;
use crate::ir::expr::NameIR;
use crate::ir::operator::Operator;
use crate::ir::stmt::StmtIR;
use crate::type_resolver::library::KnownFunction;
use crate::type_resolver::library::KnownLibrary;
use crate::type_resolver::library::NumPyFunction;
use crate::type_resolver::library::ResolvedAttributePath;
use crate::type_resolver::library::TorchFunction;

use crate::ir::expr_ir::ConstantIR;
use crate::ir::expr_ir::ExprIR;
use crate::linker::resolution_table::ResolutionTable;
use crate::linker::resolved_target::ResolvedTarget;
use crate::linker::scope_table::GlobalSymbolTable;
use crate::types::types::DType;
use crate::types::types::DimType;
use crate::types::types::GuardedType;
use crate::types::types::TensorType;
use crate::types::types::TensorTypeState;
use crate::types::types::TensorTypeState::Unresolved;
use crate::types::types::Type;

// probably won't need 'by_ref' since this struct might just be owned by the CFG pass in a later build
// !! Type resolver is only used in the CFG phase, we likely won't be calling build() standalone, rather just resolve stmt by stmt
pub struct TypeResolver<'ctx> {
    // pub by_ref: HashMap<SymbolRef, Type>,
    pub diagnostics: Vec<Diagnostic>,

    symbols: &'ctx GlobalSymbolTable,
    resolutions: &'ctx ResolutionTable,

    // both this layer and the layer above need to insert and query from it, this is fine
    function_contracts: Rc<RefCell<FunctionContractTable>>,

    solver: z3::Solver,
}

impl<'ctx> TypeResolver<'ctx> {
    pub fn new(
        symbols: &'ctx GlobalSymbolTable,
        resolutions: &'ctx ResolutionTable,
        function_contracts: Rc<RefCell<FunctionContractTable>>,
    ) -> Self {
        Self {
            // by_ref: HashMap::new(),
            symbols,
            resolutions,
            diagnostics: Vec::new(),
            solver: z3::Solver::new(),
            function_contracts,
        }
    }

    fn add_guarded_constraint(&mut self, guard: &z3::ast::Bool, constraint: &z3::ast::Bool) {
        self.solver.assert(guard.implies(constraint));
    }

    // similar to resolve_external_annotation, not the same return type
    fn resolve_known_function(
        &self,
        root: KnownLibrary,
        attrs: &[String],
    ) -> Option<KnownFunction> {
        match (root, attrs) {
            // PyTorch
            (KnownLibrary::PyTorch, [name]) if name == "tensor" => {
                Some(KnownFunction::Torch(TorchFunction::Tensor))
            }

            (KnownLibrary::PyTorch, [name]) if name == "zeros" => {
                Some(KnownFunction::Torch(TorchFunction::Zeros))
            }

            (KnownLibrary::PyTorch, [name]) if name == "ones" => {
                Some(KnownFunction::Torch(TorchFunction::Ones))
            }

            (KnownLibrary::PyTorch, [name]) if name == "empty" => {
                Some(KnownFunction::Torch(TorchFunction::Empty))
            }

            (KnownLibrary::PyTorch, [name]) if name == "arange" => {
                Some(KnownFunction::Torch(TorchFunction::Arange))
            }

            (KnownLibrary::PyTorch, [name]) if name == "reshape" => {
                Some(KnownFunction::Torch(TorchFunction::Reshape))
            }

            (KnownLibrary::PyTorch, [name]) if name == "cat" => {
                Some(KnownFunction::Torch(TorchFunction::Cat))
            }

            (KnownLibrary::PyTorch, [name]) if name == "stack" => {
                Some(KnownFunction::Torch(TorchFunction::Stack))
            }

            (KnownLibrary::PyTorch, [name]) if name == "matmul" => {
                Some(KnownFunction::Torch(TorchFunction::Matmul))
            }

            // (KnownLibrary::PyTorch, [name])
            //     if name == "mm" =>
            // {
            //     Some(KnownFunction::Torch(TorchFunction::MatrixMatrix))
            // }
            (KnownLibrary::PyTorch, [nn, functional, relu])
                if nn == "nn" && functional == "functional" && relu == "relu" =>
            {
                Some(KnownFunction::Torch(TorchFunction::Relu))
            }

            // NumPy
            (KnownLibrary::NumPy, [name]) if name == "array" => {
                Some(KnownFunction::NumPy(NumPyFunction::Array))
            }

            (KnownLibrary::NumPy, [name]) if name == "zeros" => {
                Some(KnownFunction::NumPy(NumPyFunction::Zeros))
            }

            (KnownLibrary::NumPy, [name]) if name == "ones" => {
                Some(KnownFunction::NumPy(NumPyFunction::Ones))
            }

            (KnownLibrary::NumPy, [name]) if name == "empty" => {
                Some(KnownFunction::NumPy(NumPyFunction::Empty))
            }

            (KnownLibrary::NumPy, [name]) if name == "arange" => {
                Some(KnownFunction::NumPy(NumPyFunction::Arange))
            }

            (KnownLibrary::NumPy, [name]) if name == "reshape" => {
                Some(KnownFunction::NumPy(NumPyFunction::Reshape))
            }

            (KnownLibrary::NumPy, [name]) if name == "concatenate" => {
                Some(KnownFunction::NumPy(NumPyFunction::Concatenate))
            }

            (KnownLibrary::NumPy, [name]) if name == "stack" => {
                Some(KnownFunction::NumPy(NumPyFunction::Stack))
            }

            (KnownLibrary::NumPy, [name]) if name == "matmul" => {
                Some(KnownFunction::NumPy(NumPyFunction::Matmul))
            }

            _ => None,
        }
    }

    fn resolve_name_root(&self, name: &NameIR, program_id: i64) -> Option<KnownLibrary> {
        let symbol_ref = self
            .symbols
            .lookup_by_name(program_id, name.use_scope_id, &name.id)?;

        let target = self.resolutions.imports.get(&symbol_ref)?;

        match target {
            ResolvedTarget::External { module, name: _ } => KnownLibrary::from_str(module),

            ResolvedTarget::Local(_) => None,
        }
    }

    fn resolve_attribute(
        &self,
        attr: &AttributeIR,
        program_id: i64,
    ) -> Option<ResolvedAttributePath> {
        match &*attr.value {
            ExprIR::Name(name) => Some(ResolvedAttributePath {
                root: self.resolve_name_root(name, program_id)?,
                attrs: vec![attr.attr.clone()],
            }),

            ExprIR::Attribute(inner) => {
                let mut path = self.resolve_attribute(inner, program_id)?;

                path.attrs.push(attr.attr.clone());

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
                        self.diagnostics.push(Diagnostic {
                            severity: Severity::ERROR,
                            span: list.span.clone(),
                            kind: DiagnosticKind::ShapeError,
                            message: "tensor data has inconsistent nested dimensions".to_string(),
                        });
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
            _ => DType::Unknown,
        }
    }

    // follow PyTorch numeric promotion rules
    fn resolve_common_dtype(&self, element_types: &[DType]) -> DType {
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
        let _default_dtype = DType::Unknown;
        match expr {
            ExprIR::Constant(ConstantIR::IntegerLit(_)) => {
                Some(TensorType {
                    shape: vec![],
                    dtype: DType::Int64, // TODO double check if this is the right default
                })
            }

            ExprIR::Constant(ConstantIR::FloatLit(_)) => Some(TensorType {
                shape: vec![],
                dtype: DType::Float64,
            }),

            // torch.tensor[[3, 4, 5]] etc
            ExprIR::ListExpr(_) => Some(TensorType {
                shape: self.infer_tensor_list(expr),
                dtype: self.infer_container_element_types(expr, program_id),
            }),

            // should be same logic as the list ? TODO double check
            ExprIR::TupleExpr(_) => Some(TensorType {
                shape: self.infer_tensor_list(expr),
                dtype: self.infer_container_element_types(expr, program_id),
            }),

            _ => None,
        }
    }

    // support both bare name and torch.whatever
    fn infer_tensor_dtype(&self, kw: &KeywordIR, program_id: i64) -> DType {
        if kw.arg.as_deref() != Some("dtype") {
            return DType::Unknown;
        }

        match &*kw.value {
            // e.g. dtype=float32
            ExprIR::Name(name) => match name.id.as_str() {
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
            },

            // e.g. dtype=torch.float32
            ExprIR::Attribute(attr) => {
                let Some(path) = self.resolve_attribute(attr, program_id) else {
                    return DType::Unknown;
                };

                match (path.root, path.attrs.as_slice()) {
                    (KnownLibrary::PyTorch, [name]) => match name.as_str() {
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
                    },

                    _ => DType::Unknown,
                }
            }

            _ => DType::Unknown,
        }
    }

    // get more information about the tensors, their dtype, their dimensions etc
    fn infer_torch_tensor(&mut self, call: &CallIR, program_id: i64) -> Type {
        // pytorch tensor can look like: torch.tensor(3), torch.tensor([...]), need to parse possible variants
        let Some(data_arg) = call.args.first() else {
            self.diagnostics.push(Diagnostic {
                severity: Severity::ERROR,
                span: call.span.clone(),
                kind: DiagnosticKind::TypeError,
                message: "torch.tensor requires a data argument".to_string(),
            });
            return Type::Unknown;
        };

        let Some(mut info) = self.infer_tensor_data(data_arg, program_id) else {
            return Type::Tensor(TensorTypeState::Unresolved);
        };

        let dtype = match call
            .keywords
            .iter()
            .find(|kw| kw.arg.as_deref() == Some("dtype"))
        {
            Some(kw) => self.infer_tensor_dtype(kw, program_id),
            None => info.dtype,
        };

        // maybe add some check for int and float found not good etc?
        // will see if it makes sense to do that

        info.dtype = dtype;

        Type::Tensor(TensorTypeState::Resolved(info))

        // resolve dtype, find argument "dtype" and resolve if exists else unknown dtype (? double check)
    }

    // fn require_dims_equal(
    //     &self,
    //     a: &DimType,
    //     b: &DimType,
    //     state: &mut FlowState,
    // ) -> bool {
    //     match (a, b) {
    //         (DimType::Known(a), DimType::Known(b)) => a == b,

    //         (DimType::Known(a), DimType::Symbol(b)) => {
    //             state.constraints.push(b.eq(&z3::ast::Int::from_i64(*a)));
    //             true
    //         }

    //         (DimType::Symbol(a), DimType::Known(b)) => {
    //             state.constraints.push(a.eq(&z3::ast::Int::from_i64(*b)));
    //             true
    //         }

    //         (DimType::Symbol(a), DimType::Symbol(b)) => {
    //             state.constraints.push(a.eq(b));
    //             true
    //         }

    //         _ => false,
    //     }
    // }

    fn require_dims_equal(&self, a: &DimType, b: &DimType, state: &mut FlowState) -> bool {
        let equality = match (a, b) {
            (DimType::Known(a), DimType::Known(b)) => {
                z3::ast::Int::from_i64(*a).eq(z3::ast::Int::from_i64(*b))
            }

            (DimType::Known(a), DimType::Symbol(b)) => b.eq(z3::ast::Int::from_i64(*a)),

            (DimType::Symbol(a), DimType::Known(b)) => a.eq(z3::ast::Int::from_i64(*b)),

            (DimType::Symbol(a), DimType::Symbol(b)) => a.eq(b),

            _ => return false,
        };

        self.solver.push();

        self.solver.assert(&state.guard);
        self.solver.assert(&equality);

        let result = self.solver.check();

        self.solver.pop(1);

        match result {
            z3::SatResult::Sat => {
                println!(
                    "✓ feasible: {:?} == {:?} under guard {:?}",
                    a,
                    b,
                    state.guard.simplify()
                );

                self.solver.assert(state.guard.implies(&equality));

                true
            }

            z3::SatResult::Unsat => {
                println!(
                    "✗ UNSAT — pruning path: {:?} == {:?} under guard {:?}",
                    a,
                    b,
                    state.guard.simplify()
                );

                false
            }

            z3::SatResult::Unknown => {
                // Conservatively don't prune something Z3 couldn't prove impossible.
                println!("? Z3 returned unknown");
                true
            }
        }
    }

    fn torch_shapes_compatible(
        &mut self,
        left: &Type,
        right: &Type,
        state: &mut FlowState,
    ) -> Type {
        match (left, right) {
            // Expand unions on the left.
            (Type::FlowUnion(uniontype), _) => {
                let parent_guard = state.guard.clone();
                let mut results = Vec::new();

                for guarded in uniontype {
                    let branch_guard = z3::ast::Bool::and(&[&parent_guard, &guarded.guard]);

                    if !self.is_feasible(&branch_guard) {
                        continue;
                    }

                    state.guard = branch_guard.clone();

                    let result = self.torch_shapes_compatible(&guarded.ty, right, state);

                    match result {
                        Type::FlowUnion(inner) => {
                            results.extend(inner);
                        }
                        ty => {
                            results.push(GuardedType {
                                guard: branch_guard,
                                ty,
                            });
                        }
                    }
                }

                state.guard = parent_guard;
                Type::FlowUnion(results)
            }

            (_, Type::FlowUnion(uniontype)) => {
                let parent_guard = state.guard.clone();
                let mut results = Vec::new();

                for guarded in uniontype {
                    let branch_guard = z3::ast::Bool::and(&[&parent_guard, &guarded.guard]);

                    if !self.is_feasible(&branch_guard) {
                        continue;
                    }

                    state.guard = branch_guard.clone();

                    let result = self.torch_shapes_compatible(left, &guarded.ty, state);

                    match result {
                        Type::FlowUnion(inner) => {
                            results.extend(inner);
                        }
                        ty => {
                            results.push(GuardedType {
                                guard: branch_guard,
                                ty,
                            });
                        }
                    }
                }

                state.guard = parent_guard;
                Type::FlowUnion(results)
            }

            (
                // ex, torch.Tensor was supplied as param annotation in a function's body
                Type::Tensor(Unresolved),
                Type::Tensor(Unresolved),
            ) => {
                todo!()
            }

            (
                Type::Tensor(TensorTypeState::Resolved(TensorType {
                    shape: shape_a,
                    dtype: dtype_a,
                })),
                Type::Tensor(TensorTypeState::Resolved(TensorType {
                    shape: shape_b,
                    dtype: dtype_b,
                })),
            ) => {
                if shape_a.is_empty() || shape_b.is_empty() {
                    return Type::Unknown;
                }

                let Some(result_dtype) = self.require_matmul_dtype(dtype_a, dtype_b) else {
                    return Type::Unknown;
                };

                match (shape_a.len(), shape_b.len()) {
                    // [K] @ [K] -> []
                    (1, 1) => {
                        if !self.require_dims_equal(&shape_a[0], &shape_b[0], state) {
                            return Type::Unknown;
                        }

                        Type::Tensor(TensorTypeState::Resolved(TensorType {
                            shape: Vec::new(),
                            dtype: result_dtype,
                        }))
                    }

                    // [K] @ [..., K, N] -> [..., N]
                    (1, _) => {
                        let b_rank = shape_b.len();

                        if !self.require_dims_equal(&shape_a[0], &shape_b[b_rank - 2], state) {
                            return Type::Unknown;
                        }

                        let mut result_shape = shape_b[..b_rank - 2].to_vec();

                        result_shape.push(shape_b[b_rank - 1].clone());

                        Type::Tensor(TensorTypeState::Resolved(TensorType {
                            shape: result_shape,
                            dtype: result_dtype,
                        }))
                    }

                    // [..., M, K] @ [K] -> [..., M]
                    (_, 1) => {
                        let a_rank = shape_a.len();

                        if !self.require_dims_equal(&shape_a[a_rank - 1], &shape_b[0], state) {
                            return Type::Unknown;
                        }

                        let result_shape = shape_a[..a_rank - 1].to_vec();

                        Type::Tensor(TensorTypeState::Resolved(TensorType {
                            shape: result_shape,
                            dtype: result_dtype,
                        }))
                    }

                    // [..., M, K] @ [..., K, N]
                    _ => {
                        let a_rank = shape_a.len();
                        let b_rank = shape_b.len();

                        // A[..., M, K] @ B[..., K, N]
                        if !self.require_dims_equal(
                            &shape_a[a_rank - 1],
                            &shape_b[b_rank - 2],
                            state,
                        ) {
                            return Type::Unknown;
                        }

                        let batch_a = &shape_a[..a_rank - 2];
                        let batch_b = &shape_b[..b_rank - 2];

                        let batch_rank = batch_a.len().max(batch_b.len());

                        let offset_a = batch_rank - batch_a.len();
                        let offset_b = batch_rank - batch_b.len();

                        let mut result_shape = Vec::with_capacity(batch_rank + 2);

                        for i in 0..batch_rank {
                            let dim_a = if i >= offset_a {
                                Some(&batch_a[i - offset_a])
                            } else {
                                None
                            };

                            let dim_b = if i >= offset_b {
                                Some(&batch_b[i - offset_b])
                            } else {
                                None
                            };

                            let result_dim = match (dim_a, dim_b) {
                                (None, Some(b)) => b.clone(),
                                (Some(a), None) => a.clone(),

                                (Some(a), Some(b)) => {
                                    // 1 broadcasts to the other dimension
                                    if matches!(a, DimType::Known(1)) {
                                        b.clone()
                                    } else if matches!(b, DimType::Known(1)) {
                                        a.clone()
                                    } else {
                                        if !self.require_dims_equal(a, b, state) {
                                            return Type::Unknown;
                                        }

                                        a.clone()
                                    }
                                }

                                (None, None) => unreachable!(),
                            };

                            result_shape.push(result_dim);
                        }

                        result_shape.push(shape_a[a_rank - 2].clone());
                        result_shape.push(shape_b[b_rank - 1].clone());

                        Type::Tensor(TensorTypeState::Resolved(TensorType {
                            shape: result_shape,
                            dtype: result_dtype,
                        }))
                    }
                }
            }

            _ => Type::Unknown,
        }
    }

    fn require_matmul_dtype(&self, lhs: &DType, rhs: &DType) -> Option<DType> {
        match (lhs, rhs) {
            (DType::Unknown, _) | (_, DType::Unknown) => None,

            (a, b) if a == b => Some(a.clone()),

            _ => {
                // incompatible matmul operand dtypes
                None
            }
        }
    }

    fn infer_torch_size(
        &mut self,
        args: &[ExprIR],
        program_id: i64,
        state: &mut FlowState,
    ) -> Option<Vec<DimType>> {
        let expressions: Vec<&ExprIR> = match args {
            [ExprIR::TupleExpr(tuple)] => tuple.elts.iter().collect(),
            [ExprIR::ListExpr(list)] => list.elts.iter().collect(),
            [] => return None,
            args => args.iter().collect(),
        };

        let mut shape = Vec::with_capacity(expressions.len());

        for expr in expressions {
            let dim = match expr {
                ExprIR::Constant(ConstantIR::IntegerLit(integer)) => DimType::Known(integer.value),

                _ => match self.parse_expr(expr, program_id, state) {
                    Ok(Type::Dim(dim)) => dim,
                    Ok(Type::Int) => DimType::Unknown,
                    _ => return None,
                },
            };

            shape.push(dim);
        }

        Some(shape)
    }

    fn infer_torch_matmul(
        &mut self,
        call: &CallIR,
        program_id: i64,
        state: &mut FlowState,
    ) -> ResolveResult {
        let Some(first_arg) = call.args.get(0) else {
            return Ok(Type::Unknown);
        };

        let type_first = self.parse_expr(first_arg, program_id, state)?;

        let Some(second_arg) = call.args.get(1) else {
            return Ok(Type::Unknown);
        };

        let type_second = self.parse_expr(second_arg, program_id, state)?;

        Ok(self.torch_shapes_compatible(&type_first, &type_second, state))
    }

    fn infer_torch_factory_tensor(
        &mut self,
        call: &CallIR,
        program_id: i64,
        state: &mut FlowState,
    ) -> ResolveResult {
        let Some(shape) = self.infer_torch_size(&call.args, program_id, state) else {
            return Ok(Type::Unknown);
        };

        let dtype = call
            .keywords
            .iter()
            .find(|kw| kw.arg.as_deref() == Some("dtype"))
            .map(|kw| self.infer_tensor_dtype(kw, program_id))
            .unwrap_or(DType::Float32);

        Ok(Type::Tensor(TensorTypeState::Resolved(TensorType {
            shape,
            dtype,
        })))
    }

    fn infer_torch_arange(
        &mut self,
        call: &CallIR,
        program_id: i64,
        state: &mut FlowState,
    ) -> ResolveResult {
        if call.args.is_empty() || call.args.len() > 3 {
            return Ok(Type::Unknown);
        }

        // torch.arange(end)
        // torch.arange(start, end)
        // torch.arange(start, end, step)
        let (start, end, step) = match call.args.as_slice() {
            [end] => (None, end, None),
            [start, end] => (Some(start), end, None),
            [start, end, step] => (Some(start), end, Some(step)),
            _ => unreachable!(),
        };

        // Explicit dtype wins.
        let dtype = if let Some(keyword) = call
            .keywords
            .iter()
            .find(|kw| kw.arg.as_deref() == Some("dtype"))
        {
            self.infer_tensor_dtype(keyword, program_id)
        } else {
            // Otherwise PyTorch uses the default floating dtype if any
            // start/end/step argument is floating point; int64 otherwise.
            let mut has_float = false;
            let mut valid_numeric = true;

            for expr in &call.args {
                match self.parse_expr(expr, program_id, state)? {
                    Type::Float => has_float = true,
                    Type::Int | Type::Dim(_) => {}
                    _ => valid_numeric = false,
                }
            }

            if !valid_numeric {
                DType::Unknown
            } else if has_float {
                DType::Float32
            } else {
                DType::Int64
            }
        };

        // We can resolve the exact length when the range arguments are
        // statically-known integer literals.
        let start_value = match start {
            None => Some(0),

            Some(ExprIR::Constant(ConstantIR::IntegerLit(integer))) => Some(integer.value),

            _ => None,
        };

        let end_value = match end {
            ExprIR::Constant(ConstantIR::IntegerLit(integer)) => Some(integer.value),

            _ => None,
        };

        let step_value = match step {
            None => Some(1),

            Some(ExprIR::Constant(ConstantIR::IntegerLit(integer))) => Some(integer.value),

            _ => None,
        };

        let length = match (start_value, end_value, step_value) {
            (Some(start), Some(end), Some(step)) => {
                if step == 0 {
                    return Ok(Type::Unknown);
                }

                let start = start as i128;
                let end = end as i128;
                let step = step as i128;

                let len = if step > 0 {
                    if start >= end {
                        0
                    } else {
                        ((end - start - 1) / step) + 1
                    }
                } else {
                    if start <= end {
                        0
                    } else {
                        ((start - end - 1) / -step) + 1
                    }
                };

                match i64::try_from(len) {
                    Ok(len) => DimType::Known(len),
                    Err(_) => DimType::Unknown,
                }
            }

            _ => DimType::Unknown,
        };

        Ok(Type::Tensor(TensorTypeState::Resolved(TensorType {
            shape: vec![length],
            dtype,
        })))
    }

    fn infer_torch_reshape(
        &mut self,
        call: &CallIR,
        program_id: i64,
        state: &mut FlowState,
    ) -> ResolveResult {
        // torch.reshape(input, shape)
        if call.args.len() != 2 {
            return Ok(Type::Unknown);
        }

        let input_type = self.parse_expr(&call.args[0], program_id, state);

        let Some(mut new_shape) = self.infer_torch_size(&call.args[1..], program_id, state) else {
            return Ok(Type::Unknown);
        };

        let Ok(Type::Tensor(TensorTypeState::Resolved(input))) = input_type else {
            return match input_type {
                Ok(Type::Tensor(TensorTypeState::Unresolved)) => {
                    Ok(Type::Tensor(TensorTypeState::Unresolved))
                }

                _ => Ok(Type::Unknown),
            };
        };

        let mut infer_index = None;

        for (index, dim) in new_shape.iter().enumerate() {
            if let DimType::Known(value) = dim {
                if *value == -1 {
                    if infer_index.is_some() {
                        // Only one dimension may be inferred.
                        return Ok(Type::Unknown);
                    }

                    infer_index = Some(index);
                } else if *value < 0 {
                    return Ok(Type::Unknown);
                }
            }
        }

        // Calculate the input element count when all dimensions are known.
        let input_numel = input.shape.iter().try_fold(1i64, |product, dim| match dim {
            DimType::Known(value) => product.checked_mul(*value),

            _ => None,
        });

        // Calculate the requested element count, excluding -1.
        let requested_numel = new_shape.iter().try_fold(1i64, |product, dim| match dim {
            DimType::Known(-1) => Some(product),

            DimType::Known(value) => product.checked_mul(*value),

            _ => None,
        });

        match infer_index {
            Some(index) => {
                if let (Some(input_numel), Some(requested_numel)) = (input_numel, requested_numel) {
                    if requested_numel == 0 || input_numel % requested_numel != 0 {
                        return Ok(Type::Unknown);
                    }

                    new_shape[index] = DimType::Known(input_numel / requested_numel);
                } else {
                    // The reshape is valid structurally, but the inferred
                    // dimension cannot currently be determined statically.
                    new_shape[index] = DimType::Unknown;
                }
            }

            None => {
                if let (Some(input_numel), Some(requested_numel)) = (input_numel, requested_numel) {
                    if input_numel != requested_numel {
                        return Ok(Type::Unknown);
                    }
                }
            }
        }

        Ok(Type::Tensor(TensorTypeState::Resolved(TensorType {
            shape: new_shape,
            dtype: input.dtype,
        })))
    }

    fn is_feasible(&self, guard: &z3::ast::Bool) -> bool {
        self.solver.push();
        self.solver.assert(guard);

        let result = self.solver.check();

        self.solver.pop(1);

        !matches!(result, z3::SatResult::Unsat)
    }

    fn torch_cat_types(
        &mut self,
        types: &[Type],
        dim: i64,
        state: &mut FlowState,
    ) -> ResolveResult {
        if types.is_empty() {
            return Ok(Type::Unknown);
        }

        // Expand FlowUnion inputs one at a time.
        if let Some((index, union)) = types.iter().enumerate().find_map(|(index, ty)| match ty {
            Type::FlowUnion(union) => Some((index, union)),
            _ => None,
        }) {
            let parent_guard = state.guard.clone();
            let mut results = Vec::new();

            for guarded in union {
                let branch_guard = z3::ast::Bool::and(&[&parent_guard, &guarded.guard]);

                if !self.is_feasible(&branch_guard) {
                    continue;
                }

                state.guard = branch_guard.clone();

                let mut branch_types = types.to_vec();
                branch_types[index] = guarded.ty.clone();

                let result = self.torch_cat_types(&branch_types, dim, state)?;

                match result {
                    Type::FlowUnion(inner) => {
                        results.extend(inner);
                    }

                    ty => {
                        results.push(GuardedType {
                            guard: branch_guard,
                            ty,
                        });
                    }
                }
            }

            state.guard = parent_guard;
            return Ok(Type::FlowUnion(results));
        }

        let mut tensors = Vec::with_capacity(types.len());

        for ty in types {
            match ty {
                Type::Tensor(TensorTypeState::Resolved(tensor)) => {
                    tensors.push(tensor);
                }

                Type::Tensor(TensorTypeState::Unresolved) => {
                    return Ok(Type::Tensor(TensorTypeState::Unresolved));
                }

                _ => {
                    return Ok(Type::Unknown);
                }
            }
        }

        // torch.cat permits a 1-D empty tensor of shape (0,) regardless
        // of the rank of the other tensors.
        let is_empty_1d = |tensor: &TensorType| {
            tensor.shape.len() == 1 && matches!(tensor.shape[0], DimType::Known(0))
        };

        let base = tensors.iter().find(|tensor| !is_empty_1d(tensor));

        // All tensors are the special (0,) empty tensor.
        let Some(base) = base else {
            let normalized_dim = if dim < 0 { dim + 1 } else { dim };

            if normalized_dim != 0 {
                return Ok(Type::Unknown);
            }

            let dtypes: Vec<DType> = tensors.iter().map(|tensor| tensor.dtype).collect();

            let dtype = if dtypes.iter().all(|dtype| *dtype == dtypes[0]) {
                dtypes[0]
            } else {
                self.resolve_common_dtype(&dtypes)
            };

            return Ok(Type::Tensor(TensorTypeState::Resolved(TensorType {
                shape: vec![DimType::Known(0)],
                dtype,
            })));
        };

        let rank = base.shape.len();

        // Scalar tensors cannot be concatenated.
        if rank == 0 {
            return Ok(Type::Unknown);
        }

        let normalized_dim = if dim < 0 { dim + rank as i64 } else { dim };

        if normalized_dim < 0 || normalized_dim >= rank as i64 {
            return Ok(Type::Unknown);
        }

        let dim = normalized_dim as usize;

        let mut result_shape = base.shape.clone();
        let mut cat_dim = DimType::Known(0);

        for tensor in &tensors {
            // Special empty (0,) tensors contribute no elements.
            if is_empty_1d(tensor) {
                continue;
            }

            if tensor.shape.len() != rank {
                return Ok(Type::Unknown);
            }

            for axis in 0..rank {
                if axis == dim {
                    continue;
                }

                if !self.require_dims_equal(&base.shape[axis], &tensor.shape[axis], state) {
                    return Ok(Type::Unknown);
                }
            }

            cat_dim = match (&cat_dim, &tensor.shape[dim]) {
                (DimType::Known(a), DimType::Known(b)) => DimType::Known(a + b),

                (DimType::Known(a), DimType::Symbol(b)) => {
                    DimType::Symbol(z3::ast::Int::add(&[&z3::ast::Int::from_i64(*a), b]))
                }

                (DimType::Symbol(a), DimType::Known(b)) => {
                    DimType::Symbol(z3::ast::Int::add(&[a, &z3::ast::Int::from_i64(*b)]))
                }

                (DimType::Symbol(a), DimType::Symbol(b)) => {
                    DimType::Symbol(z3::ast::Int::add(&[a, b]))
                }

                _ => DimType::Unknown,
            };
        }

        result_shape[dim] = cat_dim;

        let dtypes: Vec<DType> = tensors.iter().map(|tensor| tensor.dtype).collect();

        let dtype = if dtypes.iter().all(|dtype| *dtype == dtypes[0]) {
            dtypes[0]
        } else {
            self.resolve_common_dtype(&dtypes)
        };

        Ok(Type::Tensor(TensorTypeState::Resolved(TensorType {
            shape: result_shape,
            dtype,
        })))
    }

    fn infer_torch_cat(
        &mut self,
        call: &CallIR,
        program_id: i64,
        state: &mut FlowState,
    ) -> ResolveResult {
        let Some(tensors_arg) = call.args.first() else {
            return Ok(Type::Unknown);
        };

        if call.args.len() > 2 {
            return Ok(Type::Unknown);
        }

        let tensor_types = match tensors_arg {
            ExprIR::ListExpr(list) => list
                .elts
                .iter()
                .map(|expr| self.parse_expr(expr, program_id, state))
                .collect::<Result<Vec<_>, _>>()?,

            ExprIR::TupleExpr(tuple) => tuple
                .elts
                .iter()
                .map(|expr| self.parse_expr(expr, program_id, state))
                .collect::<Result<Vec<_>, _>>()?,

            expr => match self.parse_expr(expr, program_id, state)? {
                Type::List(types) | Type::Tuple(types) => types,
                _ => return Ok(Type::Unknown),
            },
        };

        let positional_dim = call.args.get(1);

        let keyword_dim = call
            .keywords
            .iter()
            .find(|kw| kw.arg.as_deref() == Some("dim"))
            .map(|kw| &*kw.value);

        // Specifying dim both positionally and by keyword is invalid.
        if positional_dim.is_some() && keyword_dim.is_some() {
            return Ok(Type::Unknown);
        }

        let dim_expr = positional_dim.or(keyword_dim);

        let dim = match dim_expr {
            None => 0,

            Some(ExprIR::Constant(ConstantIR::IntegerLit(integer))) => integer.value,

            Some(expr) => {
                match self.parse_expr(expr, program_id, state)? {
                    Type::Dim(DimType::Known(dim)) => dim,

                    // We know this is still a cat operation and therefore
                    // returns a tensor, but cannot determine which axis.
                    Type::Int | Type::Dim(_) => {
                        return Ok(Type::Tensor(TensorTypeState::Unresolved));
                    }

                    _ => return Ok(Type::Unknown),
                }
            }
        };

        self.torch_cat_types(&tensor_types, dim, state)
    }

    fn torch_stack_types(
        &mut self,
        types: &[Type],
        dim: i64,
        state: &mut FlowState,
    ) -> ResolveResult {
        if types.is_empty() {
            return Ok(Type::Unknown);
        }

        // Expand FlowUnion inputs one at a time.
        if let Some((index, union)) = types.iter().enumerate().find_map(|(index, ty)| match ty {
            Type::FlowUnion(union) => Some((index, union)),
            _ => None,
        }) {
            let parent_guard = state.guard.clone();
            let mut results = Vec::new();

            for guarded in union {
                let branch_guard = z3::ast::Bool::and(&[&parent_guard, &guarded.guard]);

                if !self.is_feasible(&branch_guard) {
                    continue;
                }

                state.guard = branch_guard.clone();

                let mut branch_types = types.to_vec();
                branch_types[index] = guarded.ty.clone();

                let result = self.torch_stack_types(&branch_types, dim, state)?;

                match result {
                    Type::FlowUnion(inner) => {
                        results.extend(inner);
                    }

                    ty => {
                        results.push(GuardedType {
                            guard: branch_guard,
                            ty,
                        });
                    }
                }
            }

            state.guard = parent_guard;
            return Ok(Type::FlowUnion(results));
        }

        let mut tensors = Vec::with_capacity(types.len());

        for ty in types {
            match ty {
                Type::Tensor(TensorTypeState::Resolved(tensor)) => {
                    tensors.push(tensor);
                }

                Type::Tensor(TensorTypeState::Unresolved) => {
                    return Ok(Type::Tensor(TensorTypeState::Unresolved));
                }

                _ => {
                    return Ok(Type::Unknown);
                }
            }
        }

        let first = tensors[0];
        let rank = first.shape.len();

        // stack inserts a new dimension, so valid positive dims are
        // 0..=rank. Negative dims range from -(rank + 1)..=-1.
        let normalized_dim = if dim < 0 { dim + rank as i64 + 1 } else { dim };

        if normalized_dim < 0 || normalized_dim > rank as i64 {
            return Ok(Type::Unknown);
        }

        let dim = normalized_dim as usize;

        // Unlike cat, every input tensor must have exactly the same shape.
        for tensor in tensors.iter().skip(1) {
            if tensor.shape.len() != rank {
                return Ok(Type::Unknown);
            }

            for axis in 0..rank {
                if !self.require_dims_equal(&first.shape[axis], &tensor.shape[axis], state) {
                    return Ok(Type::Unknown);
                }
            }
        }

        let mut result_shape = first.shape.clone();

        result_shape.insert(dim, DimType::Known(tensors.len() as i64));

        let dtypes: Vec<DType> = tensors.iter().map(|tensor| tensor.dtype).collect();

        let dtype = if dtypes.iter().all(|dtype| *dtype == dtypes[0]) {
            dtypes[0]
        } else {
            self.resolve_common_dtype(&dtypes)
        };

        Ok(Type::Tensor(TensorTypeState::Resolved(TensorType {
            shape: result_shape,
            dtype,
        })))
    }

    fn infer_torch_stack(
        &mut self,
        call: &CallIR,
        program_id: i64,
        state: &mut FlowState,
    ) -> ResolveResult {
        let Some(tensors_arg) = call.args.first() else {
            return Ok(Type::Unknown);
        };

        if call.args.len() > 2 {
            return Ok(Type::Unknown);
        }

        let tensor_types = match tensors_arg {
            ExprIR::ListExpr(list) => {
                let mut types = Vec::new();

                for expr in &list.elts {
                    types.push(self.parse_expr(expr, program_id, state)?);
                }

                types
            }

            ExprIR::TupleExpr(tuple) => {
                let mut types = Vec::new();

                for expr in &tuple.elts {
                    types.push(self.parse_expr(expr, program_id, state)?);
                }

                types
            }

            expr => match self.parse_expr(expr, program_id, state)? {
                Type::List(types) | Type::Tuple(types) => types,
                _ => return Ok(Type::Unknown),
            },
        };

        let positional_dim = call.args.get(1);

        let keyword_dim = call
            .keywords
            .iter()
            .find(|kw| kw.arg.as_deref() == Some("dim"))
            .map(|kw| &*kw.value);

        if positional_dim.is_some() && keyword_dim.is_some() {
            return Ok(Type::Unknown);
        }

        let dim_expr = positional_dim.or(keyword_dim);

        let dim = match dim_expr {
            None => 0,

            Some(ExprIR::Constant(ConstantIR::IntegerLit(integer))) => integer.value,

            Some(expr) => match self.parse_expr(expr, program_id, state)? {
                Type::Dim(DimType::Known(dim)) => dim,

                Type::Int | Type::Dim(_) => {
                    return Ok(Type::Tensor(TensorTypeState::Unresolved));
                }

                _ => return Ok(Type::Unknown),
            },
        };

        self.torch_stack_types(&tensor_types, dim, state)
    }

    fn infer_torch_relu(
        &mut self,
        call: &CallIR,
        program_id: i64,
        state: &mut FlowState,
    ) -> ResolveResult {
        let input = if let Some(input) = call.args.first() {
            input
        } else if let Some(keyword) = call
            .keywords
            .iter()
            .find(|kw| kw.arg.as_deref() == Some("input"))
        {
            &keyword.value
        } else {
            return Ok(Type::Unknown);
        };

        let input_type = self.parse_expr(input, program_id, state)?;

        match input_type {
            Type::Tensor(tensor) => Ok(Type::Tensor(tensor)),

            Type::FlowUnion(union) => {
                let results = union
                    .into_iter()
                    .map(|guarded| {
                        let ty = match guarded.ty {
                            Type::Tensor(tensor) => Type::Tensor(tensor),

                            _ => Type::Unknown,
                        };

                        GuardedType {
                            guard: guarded.guard,
                            ty,
                        }
                    })
                    .collect();

                Ok(Type::FlowUnion(results))
            }

            _ => Ok(Type::Unknown),
        }
    }

    fn valid_arg_count(&self, contract: &FunctionContract, supplied: usize) -> bool {
        let required = contract
            .params
            .iter()
            .filter(|param| param.default.is_none())
            .count();

        supplied >= required && supplied <= contract.params.len()
    }

    // TODO expand
    fn compatible_param_type(&self, expected: &Type, supplied: &Type) -> bool {
        match (expected, supplied) {
            // no declared information
            (Type::Unknown, _) => true,

            // generic tensor annotation accepts any tensor state
            (Type::Tensor(TensorTypeState::Unresolved), Type::Tensor(_)) => true,

            // primitives
            (Type::Int, Type::Int) => true,
            (Type::Float, Type::Float) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::String, Type::String) => true,
            (Type::Bytes, Type::Bytes) => true,
            (Type::None, Type::None) => true,

            _ => false,
        }
    }

    pub fn parse_expr(
        &mut self,
        expr: &ExprIR,
        program_id: i64,
        state: &mut FlowState,
    ) -> ResolveResult {
        match expr {
            ExprIR::Constant(ConstantIR::IntegerLit(_)) => Ok(Type::Int),
            ExprIR::Constant(ConstantIR::FloatLit(_)) => Ok(Type::Float),
            ExprIR::Constant(ConstantIR::BooleanLit(_)) => Ok(Type::Bool),
            ExprIR::Constant(ConstantIR::StringLit(_)) => Ok(Type::String),
            ExprIR::Constant(ConstantIR::NoneLit(_)) => Ok(Type::None),
            ExprIR::Constant(ConstantIR::EllipsisLit(_)) => Ok(Type::Ellipsis),
            ExprIR::Constant(ConstantIR::BytesLit(_)) => Ok(Type::Bytes),
            ExprIR::Constant(ConstantIR::ComplexLit(_)) => Ok(Type::Complex),

            // can be many things, notably torch.tensor(...)
            // so this is where we start parsing tensors, amongst other things
            ExprIR::Call(call) => {
                match &*call.func {
                    ExprIR::Name(name) => {
                        // foo(...)
                        let callee_ty = self.parse_expr(&call.func, program_id, state);

                        // TODO mark difference between user defined and built-ins
                        // for now it's just user defined, so ignore the match
                        match callee_ty {
                            Ok(Type::Function(function_id)) => {
                                println!("calling function {} -> {:?}", name.id, function_id);

                                let supplied_args = call.args.len();

                                let params = {
                                    let contracts = self.function_contracts.borrow();

                                    let contract = contracts.by_id.get(&function_id).unwrap();

                                    if !self.valid_arg_count(contract, supplied_args) {
                                        return Ok(Type::Unknown);
                                    }

                                    contract.params.clone()
                                };

                                let mut bindings = Vec::new();
                                let mut param_types = Vec::new();

                                for (arg, param) in call.args.iter().zip(params.iter()) {
                                    let supplied_ty = self.parse_expr(arg, program_id, state)?;

                                    param_types.push(supplied_ty.clone());

                                    if !self.compatible_param_type(&param.ty, &supplied_ty) {
                                        // TODO eventually emit diagnostic
                                        return Ok(Type::Unknown);
                                    }

                                    bindings.push(CallBinding {
                                        symbol_id: param.symbol_id,
                                        ty: supplied_ty,
                                    });
                                }

                                let key = FunctionSpecializationKey {
                                    function_id,
                                    params: param_types,
                                };

                                let specialized_contract = {
                                    let contracts = self.function_contracts.borrow();

                                    contracts.specialized.get(&key).cloned()
                                };

                                match specialized_contract {
                                    Some(_contract) => {
                                        // turn guarded returns into type
                                        // Ok(self.contract_)
                                        Ok(Type::Unknown)
                                    }

                                    None => Err(FunctionAnalysisRequest {
                                        program_id,
                                        function_id,
                                        bindings,
                                    }),
                                }
                            }

                            _ => Ok(Type::Unknown),
                        }
                    }

                    ExprIR::Attribute(attr) => {
                        // ex torch.attribute... <- recursive type
                        let Some(path) = self.resolve_attribute(attr, program_id) else {
                            return Ok(Type::Unknown);
                        };

                        let Some(known_function) =
                            self.resolve_known_function(path.root, &path.attrs)
                        else {
                            return Ok(Type::Unknown);
                        };

                        match known_function {
                            KnownFunction::Torch(TorchFunction::Tensor) => {
                                // infer torch.tensor(...)
                                Ok(self.infer_torch_tensor(call, program_id))
                            }

                            KnownFunction::Torch(TorchFunction::Matmul) => {
                                // infer torch.matmul(...)
                                self.infer_torch_matmul(call, program_id, state)
                            }

                            KnownFunction::Torch(TorchFunction::Zeros)
                            | KnownFunction::Torch(TorchFunction::Ones)
                            | KnownFunction::Torch(TorchFunction::Empty) => {
                                // infer torch.zeros(...)
                                self.infer_torch_factory_tensor(call, program_id, state)
                            }

                            KnownFunction::Torch(TorchFunction::Arange) => {
                                // infer torch.arange(...)
                                self.infer_torch_arange(call, program_id, state)
                            }

                            KnownFunction::Torch(TorchFunction::Reshape) => {
                                // infer torch.reshape(...)
                                self.infer_torch_reshape(call, program_id, state)
                            }

                            KnownFunction::Torch(TorchFunction::Cat) => {
                                // infer torch.cat(...)
                                self.infer_torch_cat(call, program_id, state)
                            }

                            KnownFunction::Torch(TorchFunction::Stack) => {
                                // infer torch.stack(...)
                                self.infer_torch_stack(call, program_id, state)
                            }

                            KnownFunction::Torch(TorchFunction::Relu) => {
                                // infer torch.relu(...)
                                self.infer_torch_relu(call, program_id, state)
                            }

                            // add the rest when happy with the basic examples
                            _ => Ok(Type::Unknown),
                        }
                    }

                    ExprIR::SubscriptExpr(_subscript) => {
                        // handlers[i]()
                        Ok(Type::Unknown)
                    }

                    ExprIR::Call(_inner_call) => {
                        // factory()()
                        Ok(Type::Unknown)
                    }

                    ExprIR::LambdaExpr(_lambda) => {
                        // (lambda x: x)(1)
                        Ok(Type::Unknown)
                    }

                    ExprIR::IfExp(_ifexp) => {
                        // (a if cond else b)()
                        Ok(Type::Unknown)
                    }

                    ExprIR::Name(name) => {
                        let symbol_ref = self
                            .symbols
                            .lookup_by_name(program_id, name.use_scope_id, &name.id)
                            .unwrap();

                        Ok(state
                            .by_ref
                            .get(&symbol_ref)
                            .map(|binding| binding.ty.clone())
                            .unwrap_or(Type::Unknown))
                    }

                    _ => {
                        // valid expression, but not handled yet
                        Ok(Type::Unknown)
                    }
                }
            }

            ExprIR::Name(name) => {
                let symbol_ref = self
                    .symbols
                    .lookup_by_name(program_id, name.use_scope_id, &name.id)
                    .unwrap();

                Ok(state
                    .by_ref
                    .get(&symbol_ref)
                    .map(|binding| binding.ty.clone())
                    .unwrap_or(Type::Unknown))
            }

            ExprIR::TupleExpr(tuple) => {
                let element_types = tuple
                    .elts
                    .iter()
                    .map(|element| self.parse_expr(element, program_id, state))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Type::Tuple(element_types))
            }

            ExprIR::BinOpExpr(binop) => {
                match &binop.op {
                    // OR-union -> either A or B, is this the best way to represent it?
                    Operator::BitOr => {
                        // x: int | None, "|" is the operator
                        Ok(Type::Union(vec![
                            self.parse_expr(&binop.left, program_id, state)?,
                            self.parse_expr(&binop.right, program_id, state)?,
                        ]))
                    }

                    Operator::Add => {
                        let left = self.parse_expr(&binop.left, program_id, state)?;
                        let right = self.parse_expr(&binop.right, program_id, state)?;
                        Ok(self.resolve_add(left, right))
                    }

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
                match self
                    .symbols
                    .lookup_by_name(program_id, name.use_scope_id, &name.id)
                {
                    Some(reference) => {
                        match state.by_ref.get(&reference) {
                            Some(TypedBinding {
                                binding: BindingState::Bound,
                                ty,
                            }) => Ok(ty.clone()),

                            Some(TypedBinding {
                                binding: BindingState::MaybeUnbound,
                                ty,
                            }) => {
                                // add warning here
                                Ok(ty.clone())
                            }

                            _ => Ok(Type::Unknown),
                        }
                    }

                    None => Ok(Type::Unknown),
                }
            }

            ExprIR::SliceExpr(_slice) => Ok(Type::Unknown),

            ExprIR::SubscriptExpr(_subscript) => Ok(Type::Unknown),

            ExprIR::Attribute(attribute) => {
                println!("{attribute:?}");
                Ok(Type::Unknown)
            }

            ExprIR::BoolOpExpr(_boolean) => Ok(Type::Unknown),

            ExprIR::UnaryOpExpr(_unary) => Ok(Type::Unknown),

            ExprIR::CompareExpr(_cmp) => Ok(Type::Unknown),

            _ => Ok(Type::Unknown),
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

        // println!("{:?}", left);
        // println!("{:?}", right);

        match (left, right) {
            (Type::String, Type::String) => Type::String,

            (Type::List(_a), Type::List(_b)) => {
                // Type::List(self.merge_element_types(a, b))
                Type::Unknown
            }

            (Type::Tuple(_a), Type::Tuple(_b)) => {
                // concatenate tuple type information
                Type::Unknown
            }

            (Type::Tensor(Unresolved), Type::Tensor(Unresolved)) => {
                // self.resolve_tensor_add(a, b)
                Type::Tensor(Unresolved)
            }

            (Type::Tensor(_a), Type::Tensor(_b)) => {
                // self.resolve_tensor_add(a, b)
                Type::Unknown
            }

            (Type::Tensor(_a), scalar) if scalar.is_numeric() => {
                // self.resolve_tensor_scalar_add(a, scalar)
                Type::Unknown
            }

            (scalar, Type::Tensor(_b)) if scalar.is_numeric() => {
                // self.resolve_scalar_tensor_add(scalar, b)
                Type::Unknown
            }

            (Type::Union(_items), _rhs) => {
                // self.distribute_binop_over_union(Operator::Add, items, rhs)
                Type::Unknown
            }

            (_lhs, Type::Union(_items)) => {
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
            ("torch", "Tensor") => Type::Tensor(TensorTypeState::Unresolved),

            ("torch", "Size") => Type::Dim(DimType::Unknown),

            ("torch.nn", "Parameter") => Type::Tensor(TensorTypeState::Unresolved),

            // NumPy
            ("numpy", "ndarray") => Type::Tensor(TensorTypeState::Unresolved),

            // JAX
            ("jax", "Array") => Type::Tensor(TensorTypeState::Unresolved),

            ("jax.numpy", "ndarray") => Type::Tensor(TensorTypeState::Unresolved),

            // TensorFlow
            ("tensorflow", "Tensor") => Type::Tensor(TensorTypeState::Unresolved),

            ("tensorflow", "Variable") => Type::Tensor(TensorTypeState::Unresolved),

            ("tensorflow", "SparseTensor") => Type::Tensor(TensorTypeState::Unresolved),

            ("tensorflow", "RaggedTensor") => Type::Tensor(TensorTypeState::Unresolved),

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
            ResolvedTarget::Local(_local_ref) => {
                // self.by_ref.get(local_ref).cloned().unwrap_or(Type::Unknown)
                Type::Unknown
                // TODO fix, we are missing a bit of information here
            }

            ResolvedTarget::External { module, name } => {
                self.resolve_external_annotation(module, name)
            }

            _ => Type::Unknown,
        }
    }

    pub fn parse_annotation(&self, expr: &ExprIR, program_id: i64) -> Type {
        match expr {
            ExprIR::Name(name) => match name.id.as_str() {
                "int" => Type::Int,
                "float" => Type::Float,
                "str" => Type::String,
                "bool" => Type::Bool,
                "bytes" => Type::Bytes,

                _ => {
                    // resolve user-defined/imported type name
                    // example "torch" or "numpy" if external, if local something you defined, a class, a type, etc
                    self.resolve_annotation_name(program_id, name)
                }
            },

            ExprIR::Constant(_) => Type::None,

            ExprIR::BinOpExpr(binop) if matches!(binop.op, Operator::BitOr) => {
                let left = self.parse_annotation(&binop.left, program_id);
                let right = self.parse_annotation(&binop.right, program_id);

                Type::Union(vec![left, right])
            }

            ExprIR::SubscriptExpr(_subscript) => {
                // list[int], tuple[str, int], Tensor[...], etc.
                Type::Unknown
            }

            ExprIR::Attribute(attribute) => {
                let Some(path) = self.resolve_attribute(attribute, program_id) else {
                    return Type::Unknown;
                };

                let module = if path.attrs.len() > 1 {
                    format!(
                        "{:?}.{}",
                        path.root,
                        path.attrs[..path.attrs.len() - 1].join(".")
                    )
                } else {
                    path.root.as_str().to_string()
                };

                let Some(name) = path.attrs.last() else {
                    return Type::Unknown;
                };

                self.resolve_external_annotation(&module, name)
            }

            // _ => self.resolve_annotation_path(root, attrs, program_id)
            _ => Type::Unknown, // TODO for now, but fix later
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
    ) -> ResolveResult {
        match stmt {
            StmtIR::Assign(assign_stmt) => {
                let ty = self.parse_expr(&assign_stmt.value, program_id, state);
                ty
            }

            StmtIR::AnnAssign(annassign_stmt) => {
                let annotation_type = self.parse_annotation(&annassign_stmt.annotation, program_id);

                match &annassign_stmt.value {
                    Some(value) => {
                        let value_type = self.parse_expr(value, program_id, state)?;

                        // force annotation == actual ? -> too strict, tensor(unknown) can be ok for a tensor with declared dims
                        // TODO fix in later build
                        if value_type == annotation_type {
                            Ok(annotation_type)
                        } else {
                            // TODO + emit a diagnostic warning
                            self.diagnostics.push(Diagnostic {
                                severity: Severity::ERROR,
                                span: annassign_stmt.span.clone(),
                                kind: DiagnosticKind::MismatchedAnnotationType,
                                message: format!(
                                    "annotation {:?} does not match value type {:?}",
                                    annotation_type, value_type
                                ),
                            });
                            Ok(Type::Unknown)
                        }
                    }

                    None => Ok(annotation_type),
                }
            }

            _ => {
                panic!("sound the alarm, the type resolver crashed")
            }
        }
    }
}
