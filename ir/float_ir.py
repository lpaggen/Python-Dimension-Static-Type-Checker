from common.span import SourceSpan
from .expr_ir import ExprIR
from generated import _pb2


class FloatIR(ExprIR):
    def __init__(self, value: float, span: SourceSpan = None):
        super().__init__(span=span, value=value)
        self.span = span
        self.value = value

    def __repr__(self):
        return str(self.value)

    def to_proto(self):
        return _pb2.ExprIR(
            float_lit=_pb2.FloatIR(value=self.value)
        )
