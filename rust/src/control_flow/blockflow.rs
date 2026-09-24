use core::panic;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use crate::{
    control_flow::{
        block_id::{BlockID, FunctionID}, cfg_analysis_engine::{blocked_function_analysis::BlockedFunctionAnalysis, contour_id::ContourID, flow_block_id::FlowBlockID, functioncontract_table::FunctionContractTable}, cfg_table::CfgTable, class_cfg::ClassCfg, flowstate::FlowState, function_analysis_request::FunctionAnalysisRequest, function_cfg::FunctionCfg, function_contract::{ContractParam, FunctionContract, GuardedReturn}, graph::Graph, module_cfg::ModuleCfg, terminator::Terminator,
    }, diagnostic::diagnostic::{Diagnostic, DiagnosticKind, Severity}, ir::{
        expr::{ConstantIR, ExprIR},
        operator::Operator,
        stmt::StmtIR,
    }, linker::{scope_table::GlobalSymbolTable, symbol_ref::SymbolRef}, solver::BoolExpr, type_resolver::type_resolver::TypeResolver, types::types::Type,
};

pub struct BlockFlow<'ctx> {
    pub incoming: HashMap<FlowBlockID, FlowState>,
    pub edge_states: HashMap<(FlowBlockID, FlowBlockID), FlowState>, // necessary to preserve conditional guards on branches
    pub block_out: HashMap<FlowBlockID, FlowState>,

    function_contracts: Rc<RefCell<FunctionContractTable>>,

    symbols: &'ctx GlobalSymbolTable,

    pub type_resolver: TypeResolver<'ctx>,
}

impl<'ctx> BlockFlow<'ctx> {
    pub fn new(
        type_resolver: TypeResolver<'ctx>,
        symbol_table: &'ctx GlobalSymbolTable,
        function_contracts: Rc<RefCell<FunctionContractTable>>,
    ) -> Self {
        Self {
            incoming: HashMap::new(),
            edge_states: HashMap::new(),
            function_contracts,
            block_out: HashMap::new(),
            type_resolver,
            symbols: symbol_table,
        }
    }

    fn update_successor(
        &mut self,
        contour_id: ContourID,
        graph: &Graph,
        successor: BlockID,
        queue: &mut VecDeque<BlockID>,
        queued: &mut HashSet<BlockID>,
    ) {
        let successor_block = &graph.blocks[&successor];

        let successor_id = FlowBlockID {
            contour: contour_id,
            block: successor,
        };

        // find outgoing states of current block's predecessors
        let states = successor_block
            .incoming
            .iter()
            .filter_map(|pred| self.edge_states.get(&(FlowBlockID {contour: contour_id, block: *pred}, successor_id)));

        // now merge the outgoing states of the current block's predecessors
        let merged = FlowState::merge(states);

        let changed = self
            .incoming
            .get(&successor_id)
            .map(|old| old != &merged)
            .unwrap_or(true);

        // if successor is B1 depends on B0, just set IN[B1] = merge(OUT(predecessors[B1])), that's it
        // we only want to do this if something has changed, else we run into infinite loops
        if changed {
            self.incoming.insert(successor_id, merged);

            if queued.insert(successor) {
                // just means it wasn't there, Rust returns true
                queue.push_back(successor);
            }
        }
    }

    fn to_z3_bool(&self, expr: &ExprIR, program_id: i64) -> BoolExpr {
        match expr {
            ExprIR::Constant(ConstantIR::IntegerLit(intlit)) => {
                BoolExpr::from_bool(intlit.value != 0)
            }

            ExprIR::Constant(ConstantIR::FloatLit(floatlit)) => {
                BoolExpr::from_bool(floatlit.value != 0.0)
            }

            ExprIR::Constant(ConstantIR::BooleanLit(booleanlit)) => {
                BoolExpr::from_bool(booleanlit.value)
            }

            ExprIR::Constant(ConstantIR::StringLit(stringlit)) => {
                BoolExpr::from_bool(!stringlit.value.is_empty())
            }

            ExprIR::Constant(ConstantIR::NoneLit(_)) => BoolExpr::from_bool(false),

            ExprIR::Constant(ConstantIR::EllipsisLit(_)) => BoolExpr::from_bool(true),

            ExprIR::Constant(ConstantIR::BytesLit(byteslit)) => {
                BoolExpr::from_bool(!byteslit.value.is_empty())
            }

            ExprIR::Constant(ConstantIR::ComplexLit(complexlit)) => {
                BoolExpr::from_bool(complexlit.real != 0.0 || complexlit.imag != 0.0)
            }

            // TODO expand on this, this is super basic and won't scale
            ExprIR::Name(name) => {
                let symbol_ref = self
                    .symbols
                    .lookup_by_name(program_id, name.use_scope_id, &name.id)
                    .unwrap();

                BoolExpr::new_const(format!(
                    "truthy_{}_{}::{}",
                    symbol_ref.program_id, symbol_ref.symbol_id, name.id,
                ))
            }

            ExprIR::BoolOpExpr(boolop) => {
                let guards: Vec<BoolExpr> = boolop
                    .values
                    .iter()
                    .map(|expr| self.to_z3_bool(expr, program_id))
                    .collect();

                match boolop.op {
                    Operator::And => {
                        let refs: Vec<&BoolExpr> = guards.iter().collect();
                        BoolExpr::and(&refs)
                    }

                    Operator::Or => {
                        let refs: Vec<&BoolExpr> = guards.iter().collect();
                        BoolExpr::or(&refs)
                    }

                    _ => {
                        todo!()
                    }
                }
            }

            _ => {
                // default to assume accessible branch TODO check
                todo!()
            }
        }
    }

    // fn populate_function_contract(&self, program_id: i64, function: &FunctionCfg, contract: &FunctionContract) -> FunctionContract {
    //     // first parse the ArgIR into the Type
    //     let mut arg_types: Vec<Type> = Vec::new();
    //     for arg in &function.params {
    //         match arg.kind {

    //             // simplest case, def foo(a, b), or def foo(a:int, b: int)
    //             ArgKind::PositionalOnly => {
    //                 let param_type = Type::Unknown;

    //                 let annotation = match &arg.annotation {
    //                     Some(expr) => {
    //                         self.type_resolver
    //                             .parse_annotation(expr, program_id)
    //                     }

    //                     None => Type::Unknown,
    //                 };

    //                 arg_types.push(annotation);

    //             },

    //             ArgKind::PositionalOrKeyword => todo!(),

    //             ArgKind::VarPositional => todo!(),

    //             ArgKind::KeywordOnly => todo!(),

    //             ArgKind::VarKeyword => todo!(),
    //         }

    //     }

        // !!! TODO be careful, the type should not default to unknown, we need to check the return statements for more info
        // then all the k-CFA stuff  ..
        // let return_type = match &function.returns {
        //     Some(expr) => self.type_resolver.parse_expr(expr, program_id, state),
        //     None => Type::Unknown,
        // };

        // FunctionContract { params: (), return_type, constraints: None }
    // }

    pub fn resume_from_block(
        &mut self,
        program_id: i64,
        contour_id: ContourID,
        graph: &Graph,
        block_id: BlockID,
    ) -> Result<(), BlockedFunctionAnalysis> {
        self.run_worklist(
            program_id,
            contour_id,
            graph,
            vec![block_id],
        )
    }

    pub fn run_worklist(
        &mut self,
        program_id: i64,
        contour_id: ContourID,
        graph: &Graph,
        initial_blocks: Vec<BlockID>
    ) -> Result<(), BlockedFunctionAnalysis> {

        let mut queue: VecDeque<BlockID> = VecDeque::new();
        let mut queued: HashSet<BlockID> = HashSet::new(); // prevents duplicate updates on branches

        for block_id in initial_blocks {
            queue.push_back(block_id);
            queued.insert(block_id);
        }

        // suppose this has B3
        while let Some(id) = queue.pop_front() {
            let flow_id = FlowBlockID {
                contour: contour_id,
                block: id,
            };

            queued.remove(&id);

            let block = graph.blocks.get(&id).unwrap();

            // this just gets IN[B3], which we know from merge(OUT[predecessors])
            let mut state = self.incoming[&flow_id].clone();

            for &stmt in &block.statements {
                if let Err(request) = self.analyze_stmt(stmt, &mut state, program_id) {
                    return Err(BlockedFunctionAnalysis {
                        block_id: id,
                        request,
                        program_id,
                        contour: contour_id,
                    })
                }
            }

            self.block_out.insert(flow_id, state.clone());

            let successors = graph.get_outgoing_ids(&id);

            // update true and false targets with the guard, true gets "guard" false gets "NOT guard"
            // this helps later for z3 and for error reporting, we can tell the user why something may fail
            match block.terminator.as_ref() {
                Some(Terminator::Return(Some(_))) => {}

                Some(Terminator::ForNext(fornext)) => {
                    panic!("Sorry, loops are unsupported...")
                }

                Some(Terminator::Match(match_)) => {
                    panic!("Match isn't currently supported, only if-else...")
                }

                Some(Terminator::Branch(branch)) => {
                    let z3_guard = self.to_z3_bool(branch.condition, program_id);

                    let mut true_state = state.clone();

                    let mut false_state = state.clone();

                    // parent condition AND current guard
                    true_state.guard = BoolExpr::and(&[&state.guard, &z3_guard]);

                    let not_condition = z3_guard.not();

                    false_state.guard = BoolExpr::and(&[&state.guard, &not_condition]);

                    let true_target = FlowBlockID {
                        contour: contour_id,
                        block: branch.true_target,
                    };

                    self.edge_states
                        .insert((flow_id, true_target), true_state);

                    let false_target = FlowBlockID {
                        contour: contour_id,
                        block: branch.false_target,
                    };

                    self.edge_states
                        .insert((flow_id, false_target), false_state);

                    self.update_successor(contour_id, graph, branch.true_target, &mut queue, &mut queued);

                    self.update_successor(contour_id, graph, branch.false_target, &mut queue, &mut queued);
                }

                _ => {
                    for successor in successors {
                        let successor_key = FlowBlockID {
                            contour: contour_id,
                            block: successor,
                        };

                        self.edge_states
                            .insert((flow_id, successor_key), state.clone());

                        self.update_successor(
                            contour_id,
                            graph, 
                            successor, 
                            &mut queue, 
                            &mut queued
                        );
                    }
                }
            }
        }

        Ok(())
    }

    // for every program, go one by one to resolve CFG instructions Bound, Unbound, MaybeUnbound and their type
    // we want to end up with something like: Bound(int | float), etc., so we need bound status + type inference
    pub fn analyze_body(
        &mut self,
        program_id: i64,
        contour_id: ContourID,
        graph: &Graph,
        entry_state: Rc<RefCell<FlowState>>,
    ) -> Result<(), BlockedFunctionAnalysis> {
        let entry = BlockID { id: 0 }; // start at entry always

        let entry_key = FlowBlockID {
            contour: contour_id,
            block: entry,
        };

        self.incoming.insert(entry_key, entry_state.borrow().clone());

        self.run_worklist(
            program_id,
            contour_id,
            graph,
            vec![entry],
        )
    }

    pub fn analyze_stmt(
        &mut self,
        stmt: &StmtIR,
        state: &mut FlowState,
        program_id: i64,
    ) -> Result<(), FunctionAnalysisRequest> {
        match stmt {
            StmtIR::Function(function) => {
                // let return_type = match &function.returns {
                //     Some(ret) => self.type_resolver.parse_annotation(ret, program_id),
                //     None => Type::Unknown,
                // };

                let symbol_ref = SymbolRef {
                    program_id,
                    symbol_id: function.symbol_id,
                };

                let function_id = FunctionID { id: function.symbol_id };

                state.bind(&symbol_ref, Type::Function(function_id));

                // and all the rest we need to do

                Ok(())
            }

            StmtIR::Assign(assign) => {
                let diagnostic_count = self.type_resolver.diagnostics.len();
                let value_type = self
                    .type_resolver
                    .parse_expr(&assign.value, program_id, state)?;

                if value_type == Type::Unknown
                    && self.type_resolver.diagnostics.len() == diagnostic_count
                {
                    self.type_resolver.diagnostics.push(Diagnostic::new(
                        Severity::WARNING,
                        assign.value.span(),
                        DiagnosticKind::UnknownAssignValue,
                        "could not infer the assigned value's type",
                    ));
                }

                for target in &assign.targets {
                    if let ExprIR::Name(name) = target {
                        let symbol_ref = match self.symbols.lookup_by_name(
                            program_id,
                            name.use_scope_id,
                            &name.id,
                        ) {
                            Some(symbol_ref) => symbol_ref,

                            None => {
                                // TODO diag, DO NOT FAIL SILENTLY
                                continue;
                            }
                        };

                        state.bind(&symbol_ref, value_type.clone());
                    }
                }

                Ok(())
            }

            StmtIR::AnnAssign(annassign) => {
                // println!("{annassign:?}");
                let diagnostic_count = self.type_resolver.diagnostics.len();
                let target_type = self.type_resolver.resolve_type(program_id, stmt, state, &annassign.span.clone().unwrap())?;

                if target_type == Type::Unknown
                    && self.type_resolver.diagnostics.len() == diagnostic_count
                {
                    self.type_resolver.diagnostics.push(Diagnostic::new(
                        Severity::WARNING,
                        annassign.span.clone().unwrap(),
                        DiagnosticKind::UnknownAssignValue,
                        "could not infer the assigned value's type",
                    ));
                }

                if let ExprIR::Name(name) = &annassign.target {
                    let symbol_ref = self
                        .symbols
                        .lookup_by_name(program_id, name.use_scope_id, &name.id)
                        .unwrap();

                    state.bind(&symbol_ref, target_type.clone());
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }
}
