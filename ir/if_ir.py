from .stmt_ir import StmtIR
from common.span import SourceSpan
from .expr_ir import ExprIR
from typing import List
from generated import _pb2


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

    def to_proto(self):
        proto = _pb2.IfIR(
            scope_id=self.scope_id,
            then_scope_id=self.then_scope_id,
            else_scope_id=self.else_scope_id,
        )

        proto.test.CopyFrom(self.test.to_proto())
        proto.body.extend([stmt.to_proto() for stmt in self.body])
        proto.orelse.extend([stmt.to_proto() for stmt in self.orelse])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.if_stmt.CopyFrom(proto)
        return stmt