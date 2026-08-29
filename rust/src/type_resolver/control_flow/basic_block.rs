use crate::{
    ir::stmt::StmtIR,
    type_resolver::control_flow::{block_id::BlockID, terminator::Terminator},
};

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
