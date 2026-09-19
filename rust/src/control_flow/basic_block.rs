use crate::{
    control_flow::{block_id::BlockID, terminator::Terminator},
    ir::stmt::StmtIR,
};

#[derive(Debug, Clone)]
pub struct BasicBlock<'a> {
    pub incoming: Vec<BlockID>,
    pub terminator: Option<Terminator<'a>>, // terminator owns the outgoing IDs
    pub statements: Vec<&'a StmtIR>,
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
