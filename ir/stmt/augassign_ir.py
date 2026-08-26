from .stmt_ir import StmtIR
from ir.expr.expr_ir import ExprIR
from common.span import SourceSpan
from common.operators import Operator
from generated import _pb2
from dataclasses import dataclass


@dataclass
class AugAssignIR(StmtIR):
    target: ExprIR
    op: Operator
    value: ExprIR
    span: SourceSpan | None = None
    def to_proto(self):
        proto = _pb2.AugAssignIR(op=self.op.value)

        proto.target.CopyFrom(self.target.to_proto())
        proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.augassign.CopyFrom(proto)
        return stmt
