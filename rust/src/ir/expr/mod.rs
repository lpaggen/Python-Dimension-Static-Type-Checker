pub mod attribute_ir;
pub mod await_ir;
pub mod binop_ir;
pub mod bool_ir;
pub mod boolop_ir;
pub mod bytes_ir;
pub mod call_ir;
pub mod compare_ir;
pub mod complex_ir;
pub mod comprehension_ir;
pub mod conversion_ir;
pub mod dict_ir;
pub mod ellipsis_ir;
pub mod float_ir;
pub mod identifier_ir;
pub mod ifexp_ir;
pub mod integer_ir;
pub mod interpolation_ir;
pub mod joinedstr_ir;
pub mod lambda_ir;
pub mod list_ir;
pub mod name_ir;
pub mod namedexpr_ir;
pub mod none_ir;
pub mod set_ir;
pub mod slice_ir;
pub mod starred_ir;
pub mod string_ir;
pub mod subscript_ir;
pub mod templatestr_ir;
pub mod tuple_ir;
pub mod unaryop_ir;
pub mod yield_ir;
pub mod yieldfrom_ir;

pub use attribute_ir::AttributeIR;
pub use await_ir::AwaitIR;
pub use binop_ir::BinOpIR;
pub use bool_ir::BooleanIR;
pub use boolop_ir::BoolOpIR;
pub use bytes_ir::BytesIR;
pub use call_ir::{CallIR, KeywordIR};
pub use compare_ir::CompareIR;
pub use complex_ir::ComplexIR;
pub use comprehension_ir::{CompIR, DictCompIR, GeneratorExpIR, ListCompIR, SetCompIR};
pub use conversion_ir::Conversion;
pub use dict_ir::DictIR;
pub use ellipsis_ir::EllipsisIR;
pub use float_ir::FloatIR;
pub use identifier_ir::IdentifierIR;
pub use ifexp_ir::IfExpIR;
pub use integer_ir::IntegerIR;
pub use interpolation_ir::InterpolationIR;
pub use joinedstr_ir::{FormattedValueIR, JoinedStrIR};
pub use lambda_ir::LambdaIR;
pub use list_ir::ListIR;
pub use name_ir::NameIR;
pub use namedexpr_ir::NamedExprIR;
pub use none_ir::NoneIR;
pub use set_ir::SetIR;
pub use slice_ir::SliceIR;
pub use starred_ir::StarredIR;
pub use string_ir::StringIR;
pub use subscript_ir::SubscriptIR;
pub use templatestr_ir::TemplateStrIR;
pub use tuple_ir::TupleIR;
pub use unaryop_ir::UnaryOpIR;
pub use yield_ir::YieldIR;
pub use yieldfrom_ir::YieldFromIR;

use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub enum ExprIR {
    Constant(ConstantIR),
    Name(NameIR),
    JoinedStr(JoinedStrIR),
    FormattedValue(FormattedValueIR),
    TemplateStr(TemplateStrIR),

    ListExpr(ListIR),
    TupleExpr(TupleIR),
    SliceExpr(SliceIR),
    SubscriptExpr(SubscriptIR),
    Attribute(AttributeIR),
    SetExpr(SetIR),
    DictExpr(DictIR),

    AwaitExpr(AwaitIR),
    YieldExpr(YieldIR),
    YieldFromExpr(YieldFromIR),
    InterpolationExpr(InterpolationIR),

    BinOpExpr(BinOpIR),
    BoolOpExpr(BoolOpIR),
    UnaryOpExpr(UnaryOpIR),
    CompareExpr(CompareIR),
    Call(CallIR),
    NamedExpr(NamedExprIR), // := , named NamedExpr in Python for some reason

    IfExp(IfExpIR),
    LambdaExpr(LambdaIR),

    GeneratorExp(GeneratorExpIR),
    ListComp(ListCompIR),
    DictComp(DictCompIR),
    SetComp(SetCompIR),

    StarredExpr(StarredIR),

    IdentifierExpr(IdentifierIR),
}

#[derive(Debug, Clone)]
pub enum ConstantIR {
    IntegerLit(IntegerIR),
    FloatLit(FloatIR),
    StringLit(StringIR),
    BooleanLit(BooleanIR),
    BytesLit(BytesIR),
    ComplexLit(ComplexIR),
    NoneLit(NoneIR),
    EllipsisLit(EllipsisIR),
}

impl ConstantIR {
    pub fn span(&self) -> SourceSpan {
        match self {
            ConstantIR::IntegerLit(x) => x.span.clone(),
            ConstantIR::FloatLit(x) => x.span.clone(),
            ConstantIR::StringLit(x) => x.span.clone(),
            ConstantIR::BooleanLit(x) => x.span.clone(),
            ConstantIR::BytesLit(x) => x.span.clone(),
            ConstantIR::ComplexLit(x) => x.span.clone(),
            ConstantIR::NoneLit(x) => x.span.clone(),
            ConstantIR::EllipsisLit(x) => x.span.clone(),
        }
    }
}

impl ExprIR {
    pub fn span(&self) -> SourceSpan {
        match self {
            ExprIR::Constant(c) => c.span(),
            ExprIR::Name(x) => x.span.clone(),
            ExprIR::JoinedStr(x) => x.span.clone(),
            ExprIR::FormattedValue(x) => x.span.clone(),
            ExprIR::TemplateStr(x) => x.span.clone(),
            ExprIR::ListExpr(x) => x.span.clone(),
            ExprIR::TupleExpr(x) => x.span.clone(),
            ExprIR::SliceExpr(x) => x.span.clone(),
            ExprIR::SubscriptExpr(x) => x.span.clone(),
            ExprIR::Attribute(x) => x.span.clone(),
            ExprIR::SetExpr(x) => x.span.clone(),
            ExprIR::DictExpr(x) => x.span.clone(),
            ExprIR::AwaitExpr(x) => x.span.clone(),
            ExprIR::YieldExpr(x) => x.span.clone(),
            ExprIR::YieldFromExpr(x) => x.span.clone(),
            ExprIR::InterpolationExpr(x) => x.span.clone(),
            ExprIR::BinOpExpr(x) => x.span.clone(),
            ExprIR::BoolOpExpr(x) => x.span.clone(),
            ExprIR::UnaryOpExpr(x) => x.span.clone(),
            ExprIR::CompareExpr(x) => x.span.clone(),
            ExprIR::Call(x) => x.span.clone(),
            ExprIR::NamedExpr(x) => x.span.clone(),
            ExprIR::IfExp(x) => x.span.clone(),
            ExprIR::LambdaExpr(x) => x.span.clone(),
            ExprIR::GeneratorExp(x) => x.span.clone(),
            ExprIR::ListComp(x) => x.span.clone(),
            ExprIR::DictComp(x) => x.span.clone(),
            ExprIR::SetComp(x) => x.span.clone(),
            ExprIR::StarredExpr(x) => x.span.clone(),
            ExprIR::IdentifierExpr(x) => x.span.clone(),
        }
    }
}
