from common.span import SourceSpan
from .stmt_ir import StmtIR
from typing import List
from generated import _pb2
from .expr_ir import ExprIR


class WhileLoopIR(StmtIR):
    def __init__(
        self,
        test: ExprIR,
        scope_id: int,
        body_scope_id: int,
        body: List[StmtIR],
        orelse: List[StmtIR],
        span: SourceSpan = None,
    ):
        super().__init__(span)
        self.test = test
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.body = body
        self.orelse = orelse
        self.span = span

    def to_proto(self):
        proto = _pb2.WhileLoopIR(
            scope_id=self.scope_id,
            body_scope_id=self.body_scope_id,
        )

        proto.test.CopyFrom(self.test.to_proto())
        proto.body.extend([stmt.to_proto() for stmt in self.body])
        proto.orelse.extend([stmt.to_proto() for stmt in self.orelse])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.while_loop.CopyFrom(proto)
        return stmt