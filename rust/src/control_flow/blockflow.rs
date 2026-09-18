use std::{cell::RefCell, collections::{HashMap, HashSet, VecDeque}, ops::Deref, rc::Rc};

use z3::ast::Ast;

use crate::{control_flow::{
    basic_block::BasicBlock, bindingstate::BindingState, block_id::{BlockID, FunctionID}, bound_type::TypedBinding, cfg::Cfg, cfg_table::CfgTable, class_cfg::ClassCfg, flowstate::FlowState, function_cfg::FunctionCfg, function_contract::{ContractParam, FunctionContract, GuardedReturn}, functioncontract_table::FunctionContractTable, graph::Graph, module_cfg::ModuleCfg, terminator::Terminator
}, ir::{arg::ArgKind, expr::{ConstantIR, ExprIR}, nodes::{SymbolIR, SymbolKind}, operator::Operator, stmt::StmtIR}, linker::{program_table::ProgramTable, resolution_table::{self, ResolutionTable}, scope_table::GlobalSymbolTable, symbol_ref::SymbolRef}, type_resolver::type_resolver::TypeResolver, types::types::{GuardedType, Type}};

pub struct BlockFlow<'ctx> {
    pub incoming: HashMap<BlockID, FlowState>,
    pub edge_states: HashMap<(BlockID, BlockID), FlowState>,  // necessary to preserve conditional guards on branches
    pub block_out: HashMap<BlockID, FlowState>,

    function_contracts: Rc<RefCell<FunctionContractTable>>,

    symbols: &'ctx GlobalSymbolTable,

    type_resolver: TypeResolver<'ctx>,
}

impl<'ctx> BlockFlow<'ctx> {
    pub fn new(type_resolver: TypeResolver<'ctx>, symbol_table: &'ctx GlobalSymbolTable, function_contracts: Rc<RefCell<FunctionContractTable>>) -> Self {
        Self {
            incoming: HashMap::new(),
            edge_states: HashMap::new(),
            function_contracts: function_contracts,
            block_out: HashMap::new(),
            type_resolver,
            symbols: symbol_table,
        }
    }

    fn update_successor(&mut self, graph: &Graph, successor: BlockID, queue: &mut VecDeque<BlockID>, queued: &mut HashSet<BlockID>,) {
        let successor_block = &graph.blocks[&successor];

        // find outgoing states of current block's predecessors
        let states = successor_block
            .incoming
            .iter()
            .filter_map(|pred| self.edge_states.get(&(*pred, successor)));

        // now merge the outgoing states of the current block's predecessors
        let merged = FlowState::merge(states);

        let changed = self
            .incoming
            .get(&successor)
            .map(|old| old != &merged)
            .unwrap_or(true);

        // if successor is B1 depends on B0, just set IN[B1] = merge(OUT(predecessors[B1])), that's it
        // we only want to do this if something has changed, else we run into infinite loops
        if changed {
            self.incoming.insert(successor, merged);

            if queued.insert(successor) {  // just means it wasn't there, Rust returns true
                queue.push_back(successor);
            }
        }

    }

    fn to_z3_bool(&self, expr: &ExprIR, program_id: i64) -> z3::ast::Bool {
        match expr {
            ExprIR::Constant(ConstantIR::IntegerLit(intlit)) => {
                z3::ast::Bool::from_bool(intlit.value != 0)
            }

            ExprIR::Constant(ConstantIR::FloatLit(floatlit)) => {
                z3::ast::Bool::from_bool(floatlit.value != 0.0)
            }

            ExprIR::Constant(ConstantIR::BooleanLit(booleanlit)) => {
                z3::ast::Bool::from_bool(booleanlit.value)
            }

            ExprIR::Constant(ConstantIR::StringLit(stringlit)) => {
                z3::ast::Bool::from_bool(!stringlit.value.is_empty())
            }

            ExprIR::Constant(ConstantIR::NoneLit(_)) => {
                z3::ast::Bool::from_bool(false)
            }

            ExprIR::Constant(ConstantIR::EllipsisLit(_)) => {
                z3::ast::Bool::from_bool(true)
            }

            ExprIR::Constant(ConstantIR::BytesLit(byteslit)) => {
                z3::ast::Bool::from_bool(!byteslit.value.is_empty())
            }

            ExprIR::Constant(ConstantIR::ComplexLit(complexlit)) => {
                z3::ast::Bool::from_bool(
                    complexlit.real != 0.0 || complexlit.imag != 0.0
                )
            },

            // TODO expand on this, this is super basic and won't scale
            ExprIR::Name(name) => {
                let symbol_ref = self
                    .symbols
                    .lookup_by_name(program_id, name.use_scope_id, &name.id)
                    .unwrap();

                z3::ast::Bool::new_const(format!(
                    "truthy_{}_{}",
                    symbol_ref.program_id,
                    symbol_ref.symbol_id,
                ))
            }

            ExprIR::BoolOpExpr(boolop) => {
                let guards: Vec<z3::ast::Bool> = boolop
                    .values
                    .iter()
                    .map(|expr| self.to_z3_bool(expr, program_id))
                    .collect();

                match boolop.op {
                    Operator::And => {
                        let refs: Vec<&z3::ast::Bool> = guards
                            .iter()
                            .collect();
                        z3::ast::Bool::and(&refs)
                    }

                    Operator::Or => {
                        let refs: Vec<&z3::ast::Bool> = guards
                            .iter()
                            .collect();
                        z3::ast::Bool::or(&refs)
                    }

                    _ => {
                        todo!()
                    }
                }
            },

            _ => {  // default to assume accessible branch TODO check
                todo!()
            }
        }
    }

    fn analyze_module(
        &mut self,
        program_id: i64,
        module: &ModuleCfg,
        symbols: &[SymbolIR],
    ) {
        let mut state = FlowState::new(z3::ast::Bool::from_bool(true));

        for symbol in symbols {
            let symbol_ref = SymbolRef {
                program_id,
                symbol_id: symbol.id,
            };

            state.register_unbound(&symbol_ref);
        }

        self.analyze_body(
            program_id,
            &module.graph,
            state,
        );
    }

    // fn populate_function_contract(&self, program_id: i64, function: &FunctionCfg, contract: &FunctionContract) -> FunctionContract {
    //     // first parse the ArgIR into the Type
    //     // let mut arg_types: Vec<Type> = Vec::new();
    //     // for arg in &function.params {
    //     //     match arg.kind {

    //     //         // simplest case, def foo(a, b), or def foo(a:int, b: int)
    //     //         ArgKind::PositionalOnly => {
    //     //             let param_type = Type::Unknown;

    //     //             let annotation = match &arg.annotation {
    //     //                 Some(expr) => {
    //     //                     self.type_resolver
    //     //                         .parse_annotation(expr, program_id)
    //     //                 }

    //     //                 None => Type::Unknown,
    //     //             };

    //     //             arg_types.push(annotation);

    //     //         },

    //     //         ArgKind::PositionalOrKeyword => todo!(),

    //     //         ArgKind::VarPositional => todo!(),

    //     //         ArgKind::KeywordOnly => todo!(),

    //     //         ArgKind::VarKeyword => todo!(),
    //     //     }


    //     }

    //     // !!! TODO be careful, the type should not default to unknown, we need to check the return statements for more info
    //     // then all the k-CFA stuff  .. 
    //     // let return_type = match &function.returns {
    //     //     Some(expr) => self.type_resolver.parse_expr(expr, program_id, state),
    //     //     None => Type::Unknown,
    //     // };



    //     FunctionContract { params: (), return_type, constraints: None }
    // }

    fn analyze_function(
        &mut self,
        program_id: i64,
        function_id: FunctionID,
        function: &FunctionCfg,
        symbols: &[SymbolIR],
    ) {
        let mut state = FlowState::new(z3::ast::Bool::from_bool(true));

        for symbol in symbols {
            if symbol.scope_id != function.scope_id {
                continue;
            }

            let symbol_ref = SymbolRef {
                program_id,
                symbol_id: symbol.id,
            };

            state.register_unbound(&symbol_ref);
        }

        let mut contract = FunctionContract::new();

        for param in &function.params {
            let symbol_ref = SymbolRef {
                program_id,
                symbol_id: param.symbol_id,
            };

            let param_type = match &param.annotation {
                Some(expr) => {
                    self.type_resolver
                        .parse_annotation(expr, program_id)
                }

                None => Type::Unknown,
            };

            let param_default = match &param.default {
                Some(default) => Some(*default.clone()),
                None => None
            };

            let param_contract = ContractParam {
                ty: param_type.clone(),
                default: param_default,
                kind: param.kind.clone(),
            };

            // ugly? but it works, contract is just a cheap copy made to pass to callers either way]
            // TODO double check logic here
            contract.params.push(param_contract);

            state.bind(
                &symbol_ref,
                param_type,
            );
        }

        // !! be mindful of the fact that there is whatever the user DECLARED, and what actually gets returned
        // this is the former.
        contract.declared_return_type = match &function.returns {
            Some(expr) => self.type_resolver.parse_annotation(expr, program_id),
            None => Type::Unknown,
        };

        self.analyze_body(
            program_id,
            &function.graph,
            state,
        );

        // check actual return statements, might move to a new function (if we want asyncdef etc support, lambdas maybe) TODO
        let mut returns = Vec::new();

        for (block_id, block) in &function.graph.blocks {
            let Some(Terminator::Return(ret)) = &block.terminator else {
                continue;
            };

            let Some(return_state) = self.block_out.get(block_id) else {
                continue; // unreachable
            };

            let mut return_state = return_state.clone();

            let return_type = match ret {
                Some(expr) => {
                    self.type_resolver
                        .parse_expr(expr, program_id, &mut return_state)
                }

                None => Type::None,
            };

            returns.push(GuardedReturn {
                guard: return_state.guard.simplify(),
                ty: return_type,
                constraints: return_state
                    .constraints
                    .iter()
                    .map(|c| c.simplify())
                    .collect(),
            });
        }

        contract.returns = returns;

        // TODO. + add logic for dynamic CFG reconstruction, somehow. ie enter function with weak contract -> update constraints on the fly
        // self.populate_function_contract(
        //     program_id,
        //     function,
        //     &mut contract,
        // );

        // TODO see if this makes sense
        // contract.constraints = state.constraints.clone();

        // println!("{:?}", contract);

        self.function_contracts
            .borrow_mut()
            .by_id
            .insert(function_id, contract);
    }

    // TODO further improve, this is a skeleton
    fn analyze_class(
        &mut self,
        program_id: i64,
        class: &ClassCfg,
        symbols: &[SymbolIR],
    ) {
        let mut state =
            FlowState::new(z3::ast::Bool::from_bool(true));

        for symbol in symbols {
            let symbol_ref = SymbolRef {
                program_id,
                symbol_id: symbol.id,
            };

            state.register_unbound(&symbol_ref);
        }

        self.analyze_body(
            program_id,
            &class.graph,
            state,
        );
    }

    // for every program, go one by one to resolve CFG instructions Bound, Unbound, MaybeUnbound and their type
    // we want to end up with something like: Bound(int | float), etc., so we need bound status + type inference
    pub fn analyze_body(&mut self, program_id: i64, graph: &Graph, entry_state: FlowState) {

        let entry = BlockID {id: 0};  // start at entry always

        self.incoming.insert(entry, entry_state);

        let mut queue: VecDeque<BlockID> = VecDeque::new();
        let mut queued: HashSet<BlockID> = HashSet::new();  // prevents duplicate updates on branches

        queue.push_back(entry);
        queued.insert(entry);

        // suppose this has B3
        while let Some(id) = queue.pop_front() {
            queued.remove(&id);

            let block = graph
                                            .blocks
                                            .get(&id)
                                            .unwrap();

            // this just gets IN[B3], which we know from merge(OUT[predecessors])
            let mut state = self.incoming[&id].clone();

            for &stmt in &block.statements {
                self.analyze_stmt(stmt, &mut state, program_id);
            }

            self.block_out.insert(id, state.clone());

            let successors = graph.get_outgoing_ids(&id);

            // update true and false targets with the guard, true gets "guard" false gets "NOT guard"
            // this helps later for z3 and for error reporting, we can tell the user why something may fail
            match block.terminator.as_ref() {
                Some(Terminator::Return(Some(_))) => {}

                Some(Terminator::Branch(branch)) => {

                    let z3_guard = self.to_z3_bool(&branch.condition, program_id);

                    let mut true_state = state.clone();

                    let mut false_state = state.clone();

                    // parent condition AND current guard
                    true_state.guard = z3::ast::Bool::and(&[
                        &state.guard,
                        &z3_guard,
                    ]);

                    let not_condition = z3_guard.not();

                    false_state.guard = z3::ast::Bool::and(&[
                        &state.guard,
                        &not_condition,
                    ]);

                    self.edge_states.insert(
                        (id, branch.true_target), 
                        true_state
                    );

                    self.edge_states.insert(
                        (id, branch.false_target), 
                        false_state
                    );

                    self.update_successor(
                        &graph, 
                        branch.true_target, 
                        &mut queue,
                        &mut queued
                    );

                    self.update_successor(
                        &graph, 
                        branch.false_target, 
                        &mut queue,
                        &mut queued
                    );
                },

                _ => {
                    for successor in successors {
                        self.edge_states.insert(
                            (id, successor),
                            state.clone(),
                        );

                        self.update_successor(
                            &graph, 
                            successor, 
                            &mut queue,
                            &mut queued
                        );
                    }
                }
            }
        }
    }

    pub fn analyze_stmt(
        &mut self,
        stmt: &StmtIR,
        state: &mut FlowState,
        program_id: i64,
    ) {
        match stmt {
            StmtIR::Function(function) => {
                // let return_type = match &function.returns {
                //     Some(ret) => self.type_resolver.parse_annotation(ret, program_id),
                //     None => Type::Unknown,
                // };

                let symbol_ref = self.symbols.lookup_by_name(program_id, function.scope_id, &function.name).unwrap();

                let function_id = FunctionID {id: function.id};

                state.bind(
                    &symbol_ref,
                    Type::Function(function_id));

                // and all the rest we need to do
            }

            StmtIR::Assign(assign) => {
                let value_type =
                    self.type_resolver
                        .parse_expr(&assign.value, program_id, state);

                for target in &assign.targets {
                    if let ExprIR::Name(name) = target {
                        let symbol_ref = match self.symbols.lookup_by_name(
                            program_id,
                            name.use_scope_id,
                            &name.id,
                        ) {
                            Some(symbol_ref) => {
                                symbol_ref
                            }

                            None => {
                                // TODO diag, DO NOT FAIL SILENTLY
                                continue;
                            }
                        };

                        state.bind(
                            &symbol_ref,
                            value_type.clone(),
                        );
                    }
                }
            }

            StmtIR::AnnAssign(annassign) => {
                // println!("{annassign:?}");
                let target_type = self.type_resolver.resolve_type(program_id, stmt, state);
                    if let ExprIR::Name(name) = &annassign.target {
                        let symbol_ref = self.symbols
                            .lookup_by_name(
                                program_id,
                                name.use_scope_id,
                                &name.id,
                            )
                            .unwrap();

                        state.bind(
                            &symbol_ref,
                            target_type.clone(),
                        );
                    }
            }

            _ => {}
        }
    }

    pub fn build(&mut self, cfg: &CfgTable, programs: &ProgramTable) {
        for (id, program_cfg) in &cfg.programs {
            let program = programs
                .by_id
                .get(id)
                .expect("CFG exists without corresponding ProgramIR");

            for (function_id, function_cfg) in &program_cfg.functions {
                self.analyze_function(
                    *id,
                    *function_id,
                    function_cfg,
                    &program.symbols,
                );
            }

            self.analyze_module(
                *id,
                &program_cfg.module,
                &program.symbols,
            );

            for (_class_id, class_cfg) in &program_cfg.classes {
                self.analyze_class(
                    *id,
                    class_cfg,
                    &program.symbols,
                );
            }
        }
    }
}
