from dataclasses import dataclass
from common.operators import Operator
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR

@dataclass
class UnaryOpIR(ExprIR):
    op: Operator
    operand: ExprIR
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.UnaryOpIR(op=self.op.value, operand=self.operand.to_proto())
        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())
        return _pb2.ExprIR(unaryop=proto)
