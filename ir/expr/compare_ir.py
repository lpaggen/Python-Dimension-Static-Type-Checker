from dataclasses import dataclass
from common.operators import Operator
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR

@dataclass
class CompareIR(ExprIR):
    left: ExprIR
    ops: list[Operator]
    comparators: list[ExprIR]
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.CompareIR(
            left=self.left.to_proto(),
            ops=[op.value if isinstance(op, Operator) else op for op in self.ops],
            comparators=[value.to_proto() for value in self.comparators],
        )
        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())
        return _pb2.ExprIR(compare=proto)
