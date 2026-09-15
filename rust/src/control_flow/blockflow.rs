use std::{collections::{HashMap, HashSet, VecDeque}, ops::Deref};

use crate::{control_flow::{
    basic_block::BasicBlock, bindingstate::BindingState, block_id::BlockID, bound_type::TypedBinding, cfg::Cfg, cfg_table::CfgTable, class_cfg::ClassCfg, flowstate::FlowState, function_cfg::FunctionCfg, graph::Graph, module_cfg::ModuleCfg, terminator::Terminator
}, ir::{expr::{ConstantIR, ExprIR}, nodes::{SymbolIR, SymbolKind}, operator::Operator, stmt::StmtIR}, linker::{program_table::ProgramTable, resolution_table::{self, ResolutionTable}, scope_table::GlobalSymbolTable, symbol_ref::SymbolRef}, type_resolver::type_resolver::TypeResolver, types::types::Type};

pub struct BlockFlow<'ctx> {
    pub incoming: HashMap<BlockID, FlowState>,
    pub edge_states: HashMap<(BlockID, BlockID), FlowState>,  // necessary to preserve conditional guards on branches

    symbols: &'ctx GlobalSymbolTable,

    type_resolver: TypeResolver<'ctx>,
}

impl<'ctx> BlockFlow<'ctx> {
    pub fn new(type_resolver: TypeResolver<'ctx>, symbol_table: &'ctx GlobalSymbolTable) -> Self {
        Self {
            incoming: HashMap::new(),
            edge_states: HashMap::new(),
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

    fn analyze_function(
        &mut self,
        program_id: i64,
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

        for param in &function.params {
            // find the corresponding SymbolIR
            let symbol = symbols
                .iter()
                .find(|symbol| {
                    symbol.kind == SymbolKind::Param
                        && symbol.name == param.arg
                })
                .expect("function parameter missing SymbolIR");

            let symbol_ref = SymbolRef {
                program_id,
                symbol_id: symbol.id,
            };

            let param_type = match &param.annotation {
                Some(expr) => {
                    self.type_resolver
                        .parse_annotation(expr, program_id)
                }

                None => Type::Unknown,
            };

            state.bind(
                &symbol_ref,
                param_type,
            );
        }

        self.analyze_body(
            program_id,
            &function.graph,
            state,
        );
    }

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

            let successors = graph.get_outgoing_ids(&id);

            // update true and false targets with the guard, true gets "guard" false gets "NOT guard"
            // this helps later for z3 and for error reporting, we can tell the user why something may fail
            match block.terminator.as_ref() {
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

            self.analyze_module(
                *id,
                &program_cfg.module,
                &program.symbols,
            );

            for (_function_id, function_cfg) in &program_cfg.functions {
                self.analyze_function(
                    *id,
                    function_cfg,
                    &program.symbols,
                );
            }

            for (class_id, class_cfg) in &program_cfg.classes {
                println!("\n--- CLASS {:?} ---", class_id);

                self.analyze_class(
                    *id,
                    class_cfg,
                    &program.symbols,
                );
            }
        }
    }
}
