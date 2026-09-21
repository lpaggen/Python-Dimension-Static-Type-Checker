from dataclasses import dataclass
from common.operators import Operator
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR

@dataclass
class BoolOpIR(ExprIR):
    values: list[ExprIR]
    op: Operator
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.BoolOpIR(values=[value.to_proto() for value in self.values], op=self.op.value)
        proto.span.CopyFrom(self.span.to_proto())
        return _pb2.ExprIR(boolop=proto)
