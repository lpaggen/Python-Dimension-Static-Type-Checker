from common.span import SourceSpan
from .identified_ir_node import IdentifiedIRNode
from .ir_node import IRNode
from typing import List
from generated import _pb2


class ForLoopIR(IRNode):
    def __init__(
        self,
        target: IRNode,
        iter: int,
        scope_id: int,
        body_scope_id: int,
        body: List[IRNode],
        orelse: List[IRNode],
        span=None,
    ):
        super().__init__(span)
        self.target = target
        self.iter = iter
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.body = body
        self.orelse = orelse
        self.span = span

    def to_proto(self):
        proto = _pb2.ForLoopIR(
            scope_id=self.scope_id,
            body_scope_id=self.body_scope_id,
        )

        proto.target.CopyFrom(self.target.to_proto())
        proto.iter.CopyFrom(self.iter.to_proto())
        proto.body.extend([stmt.to_proto() for stmt in self.body])
        proto.orelse.extend([stmt.to_proto() for stmt in self.orelse])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.for_loop.CopyFrom(proto)
        return stmt
