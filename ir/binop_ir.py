from common.span import SourceSpan
from common.operators import Operator
from generated import _pb2
from .expr_ir import ExprIR


class BinOpIR(ExprIR):
    def __init__(self, left: ExprIR, right: ExprIR, op: Operator, span: SourceSpan=None):
        super().__init__(span=span, value=None)
        self.op = op
        self.left = left
        self.right = right
        self.span = span

    def to_proto(self):
        proto = _pb2.BinOpIR(
            op=self.op.value,
        )

        proto.left.CopyFrom(self.left.to_proto())
        proto.right.CopyFrom(self.right.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.binop.CopyFrom(proto)
        return expr
