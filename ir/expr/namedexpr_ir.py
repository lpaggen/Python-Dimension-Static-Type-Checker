from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from dataclasses import dataclass


@dataclass
class NamedExprIR(ExprIR):
    target: ExprIR
    value: ExprIR
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.NamedExprIR()
        proto.target.CopyFrom(self.target.to_proto())
        proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.named_expr.CopyFrom(proto)
        return expr
