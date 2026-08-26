from common.span import SourceSpan
from ir.expr.expr_ir import ExprIR
from ir.arg.arg_ir import ArgIR
from dataclasses import dataclass


@dataclass
class LambdaIR(ExprIR):
    args: list[ArgIR]
    body: ExprIR
    scope_id: int
    span: SourceSpan
