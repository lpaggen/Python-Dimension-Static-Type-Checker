from common.span import SourceSpan
from .expr_ir import ExprIR
from generated import _pb2


class NoneIR(ExprIR):
    def __init__(self, span: SourceSpan = None):
        super().__init__(span=span, value=None)
        self.span = span

    def to_proto(self):
        proto = _pb2.NoneIR()
        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.none_lit.CopyFrom(proto)
        return expr
