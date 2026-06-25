from .expr_ir import ExprIR
from common.span import SourceSpan
from generated import _pb2


class SubscriptIR(ExprIR):
    def __init__(self, target: ExprIR, subscript: ExprIR, span: SourceSpan):
        super().__init__(span)
        self.target = target
        self.subscript = subscript
        self.span = span

    def to_proto(self):
        proto = _pb2.SubscriptIR()

        proto.target.CopyFrom(self.target.to_proto())
        proto.subscript.CopyFrom(self.subscript.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.subscript.CopyFrom(proto)
        return expr
