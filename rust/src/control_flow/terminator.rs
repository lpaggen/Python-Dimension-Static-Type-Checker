use crate::{
    control_flow::{block_id::BlockID, branch::Branch, fornext::Next, matcharm::Match, raise::Raise}, ir::expr::ExprIR,
};

#[derive(Debug, Clone)]
pub enum Terminator<'a> {
    Branch(Branch<'a>), // if, while, (match)

    ForNext(Next<'a>),

    Match(Match<'a>),

    Goto(BlockID), // break (exit loop), continue (back to top of loop), (match)

    Return(Option<&'a ExprIR>),

    Raise(Raise<'a>),

    Exit  // block final exit (no successors), so we don't return None
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

            Terminator::Match(match_) => {
                let mut out = Vec::new();
                for arm in &match_.arms {
                    out.push(arm.target);
                }
                out.push(match_.no_match_target);
                out
            }

            Terminator::Goto(goto) => {
                vec![*goto]
            }

            Terminator::Raise(_) 
            | Terminator::Return(_)
            | Terminator::Exit => {
                vec![]
            }
        }
    }

    // pub fn next(&self) ->  todo!()
}
