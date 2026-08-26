from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from dataclasses import dataclass


@dataclass
class IfExpIR(ExprIR):
    test: ExprIR
    body: ExprIR
    orelse: ExprIR
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.IfExprIR()

        proto.test.CopyFrom(self.test.to_proto())
        proto.body.CopyFrom(self.body.to_proto())
        proto.orelse.CopyFrom(self.orelse.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.if_expr.CopyFrom(proto)  # use your actual oneof field name
        return expr
