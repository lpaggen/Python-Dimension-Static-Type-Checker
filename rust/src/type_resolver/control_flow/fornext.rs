use crate::{ir::expr::ExprIR, type_resolver::control_flow::block_id::BlockID};

pub struct Next<'a> {
    pub iterator: &'a ExprIR,
    pub target: &'a ExprIR,
    pub hasnext_target: BlockID,
    pub empty_target: BlockID,
}
