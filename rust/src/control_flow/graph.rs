use std::collections::HashMap;

use crate::{
    control_flow::{
        basic_block::BasicBlock, block_id::{BlockID, ClassID, FunctionID}, branch::Branch, cfg::Cfg, fornext::Next, loopctx::LoopContext, matcharm::{
            Match, 
            MatchArm
        }, programcfg::ProgramCfg, raise::Raise, terminator::Terminator
    }, ir::stmt::{FunctionDefIR, StmtIR}
};

#[derive(Debug, Clone)]
pub struct Graph<'a> {
    pub blocks: HashMap<BlockID, BasicBlock<'a>>,
    current_id: usize,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        let mut cfg = Self {
            blocks: HashMap::new(),
            current_id: 0,
        };

        cfg.new_block(); // entry block = BlockID { id: 0 }

        cfg
    }

    pub fn get_outgoing_ids(&self, id: &BlockID) -> Vec<BlockID> {
        self
            .blocks
            .get(&id)
            .unwrap()
            .terminator
            .as_ref()
            .unwrap()
            .outgoing()
    }

    pub fn build(
        &mut self,
        cfg: &mut ProgramCfg<'a>,  // so we can push to modules and functions and classes 
        mut current: Vec<BlockID>,
        body: &'a [StmtIR],
        loop_ctx: Option<LoopContext>,
    ) -> Vec<BlockID> {
        for stmt in body {
            match stmt {
                StmtIR::If(if_stmt) => {
                    let header = self.new_block();
                    let then_body = self.new_block();
                    let else_body = self.new_block();

                    self.set_incoming(header, &current);

                    for id in &current {
                        self.set_terminator(*id, Terminator::Goto(header));
                    }

                    self.set_incoming(then_body, &[header]);
                    self.set_incoming(else_body, &[header]);

                    self.set_terminator(
                        header,
                        Terminator::Branch(Branch {
                            condition: if_stmt.test.as_ref(),
                            true_target: then_body,
                            false_target: else_body,
                        }),
                    );

                    let then_exits = self.build(cfg, vec![then_body], &if_stmt.body, loop_ctx);

                    let else_exits = self.build(cfg, vec![else_body], &if_stmt.orelse, loop_ctx);

                    let exits: Vec<_> = then_exits
                        .into_iter()
                        .chain(else_exits)
                        .collect();

                    if exits.is_empty() {
                        current.clear();
                    } else {
                        let join = self.new_block();

                        for exit in &exits {
                            self.set_terminator(*exit, Terminator::Goto(join));
                        }

                        self.set_incoming(join, &exits);

                        current = vec![join];
                    }
                }

                StmtIR::While(while_stmt) => {
                    let header = self.new_block();
                    let body = self.new_block();
                    let exit = self.new_block();

                    self.set_incoming(header, &current);

                    // connect incoming and header
                    for id in &current {
                        self.set_terminator(*id, Terminator::Goto(header));
                    }

                    self.set_incoming(body, &[header]);
                    self.set_incoming(exit, &[header]);

                    self.set_terminator(
                        header,
                        Terminator::Branch(Branch {
                            condition: while_stmt.test.as_ref(),
                            true_target: body,
                            false_target: exit,
                        }),
                    );

                    // need to consider the body has other statements which point elsewhere
                    let body_exits = self.build(
                        cfg,
                        vec![body],
                        &while_stmt.body,
                        Some(LoopContext { header, exit }),
                    );

                    for id in &body_exits {
                        self.set_terminator(*id, Terminator::Goto(header));
                    }

                    self.set_incoming(header, &body_exits);

                    current = vec![exit]
                }

                StmtIR::For(for_stmt) => {
                    let header = self.new_block();
                    let loop_body = self.new_block();
                    let exit = self.new_block();

                    self.set_incoming(header, &current);

                    for id in &current {
                        self.set_terminator(*id, Terminator::Goto(header));
                    }

                    self.set_incoming(loop_body, &[header]);
                    self.set_incoming(exit, &[header]);

                    self.set_terminator(
                        header,
                        Terminator::ForNext(Next {
                            iterator: for_stmt.iter.as_ref(),
                            target: for_stmt.target.as_ref(),
                            hasnext_target: loop_body,
                            empty_target: exit,
                        }),
                    );

                    let body_exits = self.build(
                        cfg,
                        vec![loop_body],
                        &for_stmt.body,
                        Some(LoopContext { header, exit }),
                    );

                    for id in &body_exits {
                        self.set_terminator(*id, Terminator::Goto(header));
                    }

                    self.set_incoming(header, &body_exits);

                    current = vec![exit];
                }

                StmtIR::Return(return_stmt) => {
                    for id in &current {
                        self.set_terminator(*id, Terminator::Return(return_stmt.value.as_deref()));
                    }

                    current.clear();
                }

                // TODO add Span to 'expect' for user to see where the issue is
                StmtIR::Break(_break_stmt) => {
                    let ctx = loop_ctx.expect("break statement outside loop");

                    self.set_incoming(ctx.exit, &current);

                    for id in &current {
                        self.set_terminator(*id, Terminator::Goto(ctx.exit));
                    }

                    current.clear();
                }

                StmtIR::Continue(_) => {
                    let ctx = loop_ctx.expect("continue outside loop");

                    self.set_incoming(ctx.header, &current);

                    for id in &current {
                        self.set_terminator(*id, Terminator::Goto(ctx.header));
                    }

                    current.clear();
                }

                // TODO double check if this is right with the Raise<Option<ExprIR>>
                StmtIR::Raise(raise_stmt) => {
                    for id in &current {
                        self.set_terminator(
                            *id,
                            Terminator::Raise(Raise {
                                exception: raise_stmt.exc.as_ref(),
                                cause: raise_stmt.cause.as_ref(),
                            }),
                        );
                    }

                    current.clear();
                }

                StmtIR::Match(match_stmt) => {
                    let header = self.new_block();
                    let no_match_target = self.new_block();

                    self.set_incoming(header, &current);

                    for id in &current {
                        self.set_terminator(*id, Terminator::Goto(header));
                    }

                    let mut arms: Vec<MatchArm> = Vec::new();
                    let mut match_exits = Vec::new();

                    for case in &match_stmt.cases {
                        let target = self.new_block();

                        self.set_incoming(target, &[header]);

                        arms.push(MatchArm {
                            pattern: &case.pattern,
                            guard: case.guard.as_ref(),
                            target,
                        });

                        let case_exits = self.build(
                            cfg,
                            vec![target],
                            &case.body,
                            loop_ctx,
                        );

                        match_exits.extend(case_exits);
                    }

                    self.set_incoming(no_match_target, &[header]);

                    self.set_terminator(
                        header,
                        Terminator::Match(Match {
                            subject: match_stmt.subject.as_ref(),
                            arms,
                            no_match_target,
                        }),
                    );

                    match_exits.push(no_match_target);

                    if match_exits.is_empty() {
                        current.clear();
                    } else {
                        let join = self.new_block();

                        for exit in &match_exits {
                            self.set_terminator(*exit, Terminator::Goto(join));
                        }

                        self.set_incoming(join, &match_exits);

                        current = vec![join];
                    }
                }

                StmtIR::Class(classdef_stmt) => {
                    for id in &current {
                        self.add_statement(*id, stmt);
                    }

                    let class_id = ClassID {
                        id: cfg.current_class_id,
                    };
                    cfg.current_class_id += 1;

                    let mut class_graph = Graph::new();

                    class_graph.build(
                        cfg,
                        vec![BlockID { id: 0 }],
                        &classdef_stmt.body,
                        None,
                    );

                    cfg.classes.insert(class_id, class_graph);
                }

                // !! this is its own CFG, with its own ID
                StmtIR::Function(functiondef_stmt) => {
                    for id in &current {
                        self.add_statement(*id, stmt);
                    }

                    let function_id = FunctionID {
                        id: cfg.current_function_id,
                    };
                    cfg.current_function_id += 1;

                    let mut function_graph = Graph::new();

                    let exits = function_graph.build(
                        cfg,
                        vec![BlockID { id: 0 }],
                        &functiondef_stmt.body,
                        None,
                    );

                    for exit in exits {
                        function_graph.set_terminator(
                            exit,
                            Terminator::Return(None),
                        );
                    }

                    cfg.functions.insert(function_id, function_graph);
                }

                // StmtIR::ExprStmt(exprstmt_ir) => {
                //     println!("{:?}", exprstmt_ir)
                // }

                // TODO make a real join
                other => { // anything else, assignments, definitions, etc, go here
                    // append `other` to current BB
                    for id in &current {
                        self.add_statement(*id, other);
                    }
                }
            }
        }

        current
    }

    fn new_block(&mut self) -> BlockID {
        let id = BlockID {
            id: self.current_id,
        };

        self.current_id += 1;

        self.blocks.insert(id, BasicBlock::new());

        id
    }

    fn set_terminator(&mut self, from: BlockID, to: Terminator<'a>) {
        self.blocks
            .get_mut(&from)
            .expect("Invalid Block ID")
            .terminator = Some(to);
    }

    fn set_incoming(&mut self, target: BlockID, incoming: &[BlockID]) {
        self.blocks
            .get_mut(&target)
            .expect("Invalid Block ID")
            .incoming
            .extend_from_slice(incoming);
    }

    fn add_statement(&mut self, block_id: BlockID, statement: &'a StmtIR) {
        self.blocks
            .get_mut(&block_id)
            .expect("Invalid Block ID")
            .statements
            .push(statement);
    }
}
