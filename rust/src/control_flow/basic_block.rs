use crate::{
    ir::stmt::StmtIR,
    control_flow::{block_id::BlockID, terminator::Terminator},
};

#[derive(Debug, Clone)]
pub struct BasicBlock<'a> {
    pub incoming: Vec<BlockID>,
    pub terminator: Option<Terminator<'a>>, // terminator owns the outgoing IDs
    pub statements: Vec<StmtIR>,
}

impl<'a> BasicBlock<'a> {
    pub fn new() -> Self {
        Self {
            incoming: Vec::new(),
            terminator: None,
            statements: Vec::new(),
        }
    }
}
