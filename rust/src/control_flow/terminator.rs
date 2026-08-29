use crate::{
    ir::expr::ExprIR,
    control_flow::{block_id::BlockID, branch::Branch, fornext::Next, raise::Raise},
};

#[derive(Debug, Clone)]
pub enum Terminator<'a> {
    Branch(Branch<'a>), // if, while, (match)

    ForNext(Next<'a>),

    Goto(BlockID), // break (exit loop), continue (back to top of loop), (match)

    Return(Option<&'a ExprIR>),

    Raise(Raise<'a>),
}

impl<'a> Terminator<'a> {
    pub fn outgoing(&self) -> Vec<BlockID> {
        match self {
            Terminator::Branch(branch) => {
                vec![branch.true_target, branch.false_target]
            }

            // can't only be a branch, need to know about target too
            Terminator::ForNext(next) => {
                vec![next.hasnext_target, next.empty_target]
            }

            Terminator::Goto(goto) => {
                vec![*goto]
            }

            Terminator::Raise(_) | Terminator::Return(_) => {
                vec![]
            }
        }
    }

    // pub fn next(&self) ->  todo!()
}
