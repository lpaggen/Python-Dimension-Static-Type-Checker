from common.span import SourceSpan
from generated import _pb2
from ir.expr_ir import ExprIR


class NamedExprIR(ExprIR):
    def __init__(
        self,
        target: str,
        value: ExprIR,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.target = target
        self.value = value

    def to_proto(self):
        proto = _pb2.NamedExprIR(target=self.target)

        proto.value.CopyFrom(self.value.to_proto())

        proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.named_expr.CopyFrom(proto)
        return expr
