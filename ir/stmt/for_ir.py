from common.span import SourceSpan
from ir.ir_node import IRNode
from .stmt_ir import StmtIR, stmt_to_proto
from typing import List
from generated import _pb2
from dataclasses import dataclass


@dataclass
class ForIR(StmtIR):
    target: IRNode
    iter: IRNode
    scope_id: int
    body_scope_id: int
    body: List[IRNode]
    orelse: List[IRNode]
    span: SourceSpan | None = None
    def to_proto(self):
        proto = _pb2.ForLoopIR(
            scope_id=self.scope_id,
            body_scope_id=self.body_scope_id,
        )

        proto.target.CopyFrom(self.target.to_proto())
        proto.iter.CopyFrom(self.iter.to_proto())
        proto.body.extend([stmt_to_proto(stmt) for stmt in self.body])
        proto.orelse.extend([stmt_to_proto(stmt) for stmt in self.orelse])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.for_loop.CopyFrom(proto)
        return stmt
