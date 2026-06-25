from .ir_node import IRNode
from common.span import SourceSpan
from common.operators import Operator
from .bool_ir import BooleanIR
from generated import _pb2


class BoolOpIR(IRNode):
    def __init__(
        self, left: BooleanIR, right: BooleanIR, op: Operator, span: SourceSpan = None
    ):
        super().__init__(span=span)
        self.span = span
        self.left = left
        self.right = right
        self.op = op

    def to_proto(self):
        proto = _pb2.BoolOpIR(
            op=self.op,
        )

        proto.values.extend([v.to_proto() for v in self.values])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.boolop.CopyFrom(proto)
        return expr
