from common.span import SourceSpan
from .expr_ir import ExprIR


class IntegerIR(ExprIR):
    def __init__(self, value: int, span=None):
        super().__init__(span=span, value=value)
        self.span = span
        self.value = value

    def __repr__(self):
        return str(self.value)
