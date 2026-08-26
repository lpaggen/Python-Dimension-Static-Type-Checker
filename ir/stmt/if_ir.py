from .stmt_ir import StmtIR, stmt_to_proto
from common.span import SourceSpan
from ir.expr.expr_ir import ExprIR
from typing import List
from generated import _pb2
from dataclasses import dataclass


@dataclass
class IfIR(StmtIR):
    test: ExprIR
    scope_id: int
    then_scope_id: int
    else_scope_id: int
    body: List[StmtIR]
    orelse: List[StmtIR]
    span: SourceSpan
    def to_proto(self):
        proto = _pb2.IfIR(
            scope_id=self.scope_id,
            then_scope_id=self.then_scope_id,
            else_scope_id=self.else_scope_id,
        )

        proto.test.CopyFrom(self.test.to_proto())
        proto.body.extend([stmt_to_proto(stmt) for stmt in self.body])
        proto.orelse.extend([stmt_to_proto(stmt) for stmt in self.orelse])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.if_stmt.CopyFrom(proto)
        return stmt
