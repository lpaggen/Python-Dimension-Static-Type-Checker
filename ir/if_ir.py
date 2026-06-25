from .stmt_ir import StmtIR
from common.span import SourceSpan
from .expr_ir import ExprIR
from typing import List


class IfIR(StmtIR):
    def __init__(
        self,
        test: ExprIR,
        scope_id: int,
        then_scope_id: int,
        else_scope_id: int,
        body: List[StmtIR],
        orelse: List[StmtIR],
        span: SourceSpan,
    ):
        super().__init__(span=span)
        test = (test,)
        scope_id = (scope_id,)
        then_scope_id = (then_scope_id,)
        else_scope_id = (else_scope_id,)
        body = (body,)
        orelse = (orelse,)
