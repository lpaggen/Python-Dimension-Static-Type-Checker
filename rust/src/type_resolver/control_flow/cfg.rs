use std::collections::HashMap;

use crate::{ir::stmt::{StmtIR, continue_ir}, type_resolver::control_flow::{basic_block::BasicBlock, block_id::BlockID, terminator::Terminator}};

pub struct Cfg {
    pub blocks: HashMap<BlockID, BasicBlock>,
    current_id: usize,
}

// !! CFG just wants to BUILD the graph, doesn't care WHICH path execution takes, only models all paths
impl Cfg {
    pub fn build(
        &mut self,
        current: BlockID,
        body: &[StmtIR],
    ) -> Option<BlockID> {
        for stmt in body {
            match stmt {
                StmtIR::If(if_stmt) => {
                    // finish current BB with Branch(...)
                    // recursively build then/else bodies
                }

                StmtIR::While(while_stmt) => {
                    // create loop header/body/exit
                    // header ends in Branch(...)
                    // body ends in Goto(header)
                }

                StmtIR::For(for_stmt) => {
                    let header = self.new_block();  // make new block ID to go back to header or onto next block
                    let exit = self.new_block();

                    // current = exit
                }

                StmtIR::Return(return_stmt) => {
                    // current BB ends in Return(...)
                }

                StmtIR::Break(break_stmt) => {
                    // current BB ends in Goto(loop_exit)
                }

                StmtIR::Continue(continue_stmt) => {
                    // current BB ends in Goto(loop_header)
                }

                StmtIR::Raise(raise_stmt) => {
                    // current BB ends in Raise(...)
                }

                _other => {  // anything else, assignments, definitions, etc, go here
                    // append `other` to current BB
                }
            }
        }

        Some(BlockID {
            id: 0
        })
    }

    fn new_block(&mut self) -> BlockID {
        let id = BlockID{
            id: self.current_id
        };

        self.current_id += 1;

        self.blocks.insert(id, BasicBlock::new());

        id
    }
}
