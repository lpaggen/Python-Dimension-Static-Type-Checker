from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR


@dataclass
class SubscriptIR(ExprIR):
    value: ExprIR
    slice: ExprIR
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.SubscriptIR(value=self.value.to_proto(), slice=self.slice.to_proto())
        proto.span.CopyFrom(self.span.to_proto())
        return _pb2.ExprIR(subscript=proto)
