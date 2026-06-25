from .stmt_ir import StmtIR
from .expr_ir import ExprIR
from common.span import SourceSpan
from common.operators import Operator
from generated import _pb2


class AugAssignIR(StmtIR):
    def __init__(self, target: str, op: Operator, value=ExprIR, span: SourceSpan = None):
        super().__init__(span=span)
        self.span = span
        self.target = target
        self.op = op
        self.value = value

    def to_proto(self):
        proto = _pb2.AugAssignIR(
            op=self.op.value
        )

        proto.target.CopyFrom(self.target.to_proto())
        proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.augassign.CopyFrom(proto)
        return stmt
