pub mod assert_ir;
pub mod asyncfor_ir;
pub mod asyncfunctiondef_ir;
pub mod asyncwith_ir;
pub mod augassign_ir;
pub mod binding_ir;
pub mod break_ir;
pub mod classdef_ir;
pub mod continue_ir;
pub mod decl_ir;
pub mod delete_ir;
pub mod excepthandler_ir;
pub mod exprstmt_ir;
pub mod for_ir;
pub mod functiondef_ir;
pub mod global_ir;
pub mod if_ir;
pub mod import_ir;
pub mod match_ir;
pub mod nonlocal_ir;
pub mod pass_ir;
pub mod raise_ir;
pub mod try_ir;
pub mod trystar_ir;
pub mod typealias_ir;
pub mod while_ir;
pub mod with_ir;
pub mod withitem_ir;
pub mod assign_ir;
pub mod annassign_ir;

pub use assert_ir::AssertIR;
pub use asyncfor_ir::AsyncForIR;
pub use asyncfunctiondef_ir::AsyncFunctionDefIR;
pub use asyncwith_ir::AsyncWithIR;
pub use augassign_ir::AugAssignIR;
pub use break_ir::BreakIR;
pub use classdef_ir::ClassDefIR;
pub use continue_ir::ContinueIR;
pub use delete_ir::DeleteIR;
pub use excepthandler_ir::ExceptHandlerIR;
pub use exprstmt_ir::ExprStmtIR;
pub use for_ir::ForIR;
pub use functiondef_ir::{FunctionDefIR, ReturnIR};
pub use global_ir::GlobalIR;
pub use if_ir::IfIR;
pub use import_ir::{ImportIR, ImportKind};
pub use match_ir::{MatchCaseIR, MatchIR};
pub use nonlocal_ir::NonlocalIR;
pub use pass_ir::PassIR;
pub use raise_ir::RaiseIR;
pub use try_ir::TryIR;
pub use trystar_ir::TryStarIR;
pub use typealias_ir::TypeAliasIR;
pub use while_ir::WhileIR;
pub use with_ir::WithIR;
pub use withitem_ir::WithItemIR;
pub use assign_ir::AssignIR;
pub use annassign_ir::AnnAssignIR;


#[derive(Debug, Clone)]
pub enum StmtIR {
    Assign(AssignIR),
    AnnAssign(AnnAssignIR),
    ExprStmt(ExprStmtIR),
    AugAssign(AugAssignIR),
    If(IfIR),
    While(WhileIR),
    For(ForIR),
    Function(FunctionDefIR),
    Class(ClassDefIR),
    Import(ImportIR),
    Return(ReturnIR),
    Match(MatchIR),
    Delete(DeleteIR),
    Assert(AssertIR),
    Raise(RaiseIR),
    AsyncFor(AsyncForIR),
    AsyncFunctionDef(AsyncFunctionDefIR),

    Global(GlobalIR),
    Nonlocal(NonlocalIR),

    Pass(PassIR),
    Break(BreakIR),
    Continue(ContinueIR),

    With(WithIR),
    AsyncWith(AsyncWithIR),

    Try(TryIR),
    TryStar(TryStarIR),

    TypeAlias(TypeAliasIR),
}
