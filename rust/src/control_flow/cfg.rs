use std::collections::HashMap;

use crate::{
    control_flow::{
        basic_block::BasicBlock, block_id::BlockID, branch::Branch, fornext::Next, loopctx::LoopContext, matcharm::{Match, MatchArm}, raise::Raise, terminator::Terminator,
    }, ir::stmt::StmtIR,
};

pub struct Cfg<'a> {
    pub blocks: HashMap<BlockID, BasicBlock<'a>>,
    current_id: usize,
}

// !! CFG just wants to BUILD the graph, doesn't care WHICH path execution takes, only models all paths
// TERMINATOR OWNS THE OUTGOING EDGES !!!!!!!!
// TODO impl support for while/for 'orelse' field
impl<'a> Cfg<'a> {
    pub fn new() -> Self {
        let mut cfg = Self {
            blocks: HashMap::new(),
            current_id: 0,
        };

        cfg.new_block(); // entry block = BlockID { id: 0 }

        cfg
    }

    pub fn build(
        &mut self,
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

                    for id in current {
                        self.set_terminator(id, Terminator::Goto(header));
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

                    let then_exits = self.build(vec![then_body], &if_stmt.body, loop_ctx);

                    let else_exits = self.build(vec![else_body], &if_stmt.orelse, loop_ctx);

                    current = then_exits.into_iter().chain(else_exits).collect();
                }

                StmtIR::While(while_stmt) => {
                    let header = self.new_block();
                    let body = self.new_block();
                    let exit = self.new_block();

                    self.set_incoming(header, &current);

                    // connect incoming and header
                    for id in current {
                        self.set_terminator(id, Terminator::Goto(header));
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

                    current = match_exits;
                }

                StmtIR::Class(classdef_stmt) => {
                    println!("TODO CLASS")
                }

                StmtIR::Function(functiondef_stmt) => {
                    println!("TODO FUNCTION")
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
