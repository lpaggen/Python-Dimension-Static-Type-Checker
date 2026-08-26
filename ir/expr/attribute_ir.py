from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR


@dataclass
class AttributeIR(ExprIR):
    value: ExprIR
    attr: str
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.AttributeExprIR(attr=self.attr, value=self.value.to_proto())
        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())
        return _pb2.ExprIR(attribute=proto)
