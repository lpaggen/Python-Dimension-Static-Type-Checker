use crate::{
    control_flow::block_id::BlockID, 
    ir::{
        expr::ExprIR, 
        nodes::PatternIR
    }
};

#[derive(Debug, Clone)]
pub struct Match<'a> {
    pub subject: &'a ExprIR,
    pub arms: Vec<MatchArm<'a>>,
    pub no_match_target: BlockID,
}

#[derive(Debug, Clone)]
pub struct MatchArm<'a> {
    pub pattern: &'a PatternIR,
    pub guard: Option<&'a ExprIR>,

    pub target: BlockID,
}
