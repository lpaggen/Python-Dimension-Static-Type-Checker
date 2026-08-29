use std::collections::HashMap;

use crate::{
    ir::stmt::StmtIR,
    control_flow::{
        basic_block::BasicBlock, block_id::BlockID, branch::Branch, fornext::Next,
        loopctx::LoopContext, raise::Raise, terminator::Terminator,
    },
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

                    for id in current {
                        self.set_terminator(id, Terminator::Goto(header));
                    }

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

                    // connect incoming and header
                    for id in current {
                        self.set_terminator(id, Terminator::Goto(header));
                    }

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

                    current = vec![exit]
                }

                StmtIR::For(for_stmt) => {
                    let header = self.new_block();
                    let loop_body = self.new_block();
                    let exit = self.new_block();

                    for id in current {
                        self.set_terminator(id, Terminator::Goto(header));
                    }

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
                    for id in &current {
                        self.set_terminator(*id, Terminator::Goto(ctx.exit));
                    }

                    current.clear();
                }

                StmtIR::Continue(_) => {
                    let ctx = loop_ctx.expect("continue outside loop");

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

                _other => { // anything else, assignments, definitions, etc, go here
                    // append `other` to current BB
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
}
