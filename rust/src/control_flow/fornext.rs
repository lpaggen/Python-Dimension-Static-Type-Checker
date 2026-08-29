use crate::{ir::expr::ExprIR, control_flow::block_id::BlockID};

#[derive(Debug, Clone)]
pub struct Next<'a> {
    pub iterator: &'a ExprIR,
    pub target: &'a ExprIR,
    pub hasnext_target: BlockID,
    pub empty_target: BlockID,
}
